use std::env;

use anyhow::{Error, Ok, Result};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

pub fn create_jwt(user_id: i64) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: user_id,
        exp: 24 * 60 * 60 * 1000,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
#[derive(Deserialize)]
pub struct AccessClaims {
    pub sub: i64,
    pub roles: Vec<String>,
    pub perms: Vec<String>,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: i64, // user_id
    pub roles: Vec<String>,
    pub perms: Vec<String>, // 权限列表
    pub iat: i64,
    pub exp: i64,
    pub jti: String, // token id，可加入黑名单
}

pub fn create_access_token(
    secret: &[u8],
    user_id: i64,
    roles: Vec<String>,
    perms: Vec<String>,
    expires_in_secs: i64,
) -> Result<String, Error> {
    let now = Utc::now().timestamp();
    let jti = Uuid::new_v4().to_string();

    let claims = AccessTokenClaims {
        sub: user_id,
        roles,
        perms,
        iat: now,
        exp: now + expires_in_secs, // 15 min
        jti,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;
    Ok(token)
}

pub fn verify_access_token(secret: &[u8], token: &str) -> Result<AccessTokenClaims, Error> {
    let decoding_key = DecodingKey::from_secret(secret);
    let token_data = decode::<AccessTokenClaims>(token, &decoding_key, &Validation::default())?;
    Ok(token_data.claims)
}
