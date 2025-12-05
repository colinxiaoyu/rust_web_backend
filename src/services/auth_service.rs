use crate::{
    auth::jwt::{encode_claims, make_claims},
    repositories::user_repo::{
        exist_by_username, get_user_by_id, get_user_by_username, register_by_username_password_hash,
    },
    state::AppState,
    utils::{
        hash::hash_password,
        redis_keys::{blacklist_key, refresh_key, session_key, user_sessions_key},
    },
};
use anyhow::{Ok, Result, bail};
use bcrypt::verify;
use deadpool_redis::redis::AsyncCommands;

pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
    pub user: crate::models::user::UserResponse,
}

/// 登录：
// 1) 验证用户名密码
// 2) 生成 access Claims (短期), refresh Claims (长期)
// 3) 存 access-session 到 redis: session:{jti} -> {user_id}  TTL = access_ttl
// 4) 存 refresh 到 redis: refresh:{jti} -> {user_id} TTL = refresh_ttl
// 5) 在 user:{user_id}:sessions SET 添加 jti（用于多端管理）
pub async fn login(username: &str, password: &str, state: &AppState) -> Result<LoginResult> {
    let user = get_user_by_username(&state.db, username)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    if user.disabled {
        anyhow::bail!("user disabled");
    }

    if !verify(password, &user.password_hash)? {
        anyhow::bail!("invalid credentials");
    }

    // create access token
    let access_claims = make_claims(user.id, state.session_ttl_secs);
    let access_token = encode_claims(&state.jwt_secret, &access_claims)?;

    // create refresh token (longer TTL) — we reuse Claims but longer
    let refresh_claims = make_claims(user.id, state.refresh_ttl_secs);
    let refresh_token = encode_claims(&state.jwt_secret, &refresh_claims)?;

    // store in redis
    let mut conn = state.redis.get().await?;
    // store session access
    let s_key = session_key(&access_claims.jti);
    let r_key = refresh_key(&refresh_claims.jti);
    // store minimal info; could be json
    let _: () = conn
        .set_ex(&s_key, user.id, state.session_ttl_secs as usize)
        .await?;
    let _: () = conn
        .set_ex(&r_key, user.id, state.refresh_ttl_secs as usize)
        .await?;

    // add jti into user's session set
    let user_s_key = user_sessions_key(user.id);
    let _: () = conn.sadd(&user_s_key, &access_claims.jti).await?;
    // set TTL on the set slightly longer than refresh ttl
    let _: () = conn
        .expire(&user_s_key, state.refresh_ttl_secs as usize)
        .await?;

    // enforce max sessions per user (simple LRU not implemented here)
    let size: i64 = conn.scard(&user_s_key).await?;
    let size = size as usize;

    if size > state.max_sessions_per_user {
        let old_jti: Option<String> = conn.spop(&user_s_key).await?;
        // pop one jti and blacklisting it — simple approach
        if let Some(old_jti) = old_jti {
            let black = blacklist_key(&old_jti);
            let _: () = conn
                .set_ex(&black, "1", state.refresh_ttl_secs as usize)
                .await?;
            // also remove its session key
            let _: () = conn.del(session_key(&old_jti)).await?;
        }
    }

    Ok(LoginResult {
        access_token,
        refresh_token,
        user: (&user).into(),
    })
}

/// 刷新 token：提供 refresh_token -> 验证 -> 生成新的 access + rotate refresh (选做)
pub async fn refresh_tokens(refresh_token: &str, state: &AppState) -> Result<LoginResult> {
    use crate::auth::jwt::decode_claims;
    let claims = decode_claims(&state.jwt_secret, refresh_token)?;
    // check blacklist
    let mut conn = state.redis.get().await?;
    let black_key = blacklist_key(&claims.jti);
    let is_black: Option<String> = conn.get(&black_key).await.ok();
    if is_black.is_some() {
        anyhow::bail!("token blacklisted");
    }

    // check refresh key exists
    let r_key = refresh_key(&claims.jti);
    let user_id_opt: Option<i64> = conn.get(&r_key).await.ok();
    let user_id = user_id_opt.ok_or_else(|| anyhow::anyhow!("refresh expired"))?;

    // optional: rotate refresh token — create new refresh claim and blacklist old
    let new_refresh_claims = make_claims(user_id, state.refresh_ttl_secs);
    let new_refresh_token = encode_claims(&state.jwt_secret, &new_refresh_claims)?;
    // store new refresh
    let _: () = conn
        .set_ex(
            refresh_key(&new_refresh_claims.jti),
            user_id,
            state.refresh_ttl_secs as usize,
        )
        .await?;
    // blacklist old refresh
    let _: () = conn
        .set_ex(black_key, "1", state.refresh_ttl_secs as usize)
        .await?;
    let _: () = conn.del(&r_key).await?;

    // create access
    let access_claims = make_claims(user_id, state.session_ttl_secs);
    let access_token = encode_claims(&state.jwt_secret, &access_claims)?;
    let _: () = conn
        .set_ex(
            session_key(&access_claims.jti),
            user_id,
            state.session_ttl_secs as usize,
        )
        .await?;
    let user_s_key = user_sessions_key(user_id);
    let _: () = conn.sadd(&user_s_key, &access_claims.jti).await?;
    let _: () = conn
        .expire(&user_s_key, state.refresh_ttl_secs as usize)
        .await?;

    let user = get_user_by_id(&state.db, user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    Ok(LoginResult {
        access_token,
        refresh_token: new_refresh_token,
        user: (&user).into(),
    })
}

/// 登出：将 session jti 与 refresh jti 全部加入黑名单或删除 redis session
pub async fn logout_all(user_id: i64, state: &AppState) -> Result<()> {
    let mut conn = state.redis.get().await?;
    let user_s_key = user_sessions_key(user_id);
    let jtis: Vec<String> = conn.smembers(&user_s_key).await?;
    for jti in jtis.iter() {
        let _: () = conn.del(session_key(jti)).await?;
        let black = blacklist_key(jti);
        let _: () = conn
            .set_ex(&black, "1", state.refresh_ttl_secs as usize)
            .await?;
    }
    let _: () = conn.del(&user_s_key).await?;
    Ok(())
}

pub async fn register(username: &str, password: &str, state: &AppState) -> Result<i64> {
    let exists = exist_by_username(&state.db, username)
        .await?
        .unwrap_or(false);
    if exists {
        bail!("username already exists")
    }

    let password_hash = hash_password(password);
    let id: i64 = register_by_username_password_hash(&state.db, username, &password_hash)
        .await?
        .unwrap_or(0);
    Ok(id)
}
