use crate::utils::redis_keys::{blacklist_key, session_key, user_permissions_key};
use crate::{
    auth::jwt::decode_claims, repositories::permission_repo::get_permissions_for_user,
    state::AppState,
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use deadpool_redis::redis::AsyncCommands;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            // allow public endpoints by checking extension or route path (you can refine)
            // get auth header
            let auth_hdr = match req.headers().get("Authorization") {
                Some(v) => v.to_str().unwrap_or(""),
                None => {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body("Missing Authorization".into())
                        .unwrap());
                }
            };
            if !auth_hdr.starts_with("Bearer ") {
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("Invalid token".into())
                    .unwrap());
            }
            let token = &auth_hdr[7..];

            // decode claims
            let state = req.extensions().get::<AppState>().unwrap().clone();
            let claims = match decode_claims(&state.jwt_secret, token) {
                Ok(c) => c,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body("Invalid token".into())
                        .unwrap());
                }
            };

            // redis checks: blacklist and session exist
            let conn = state
                .redis
                .get()
                .await
                .map_err(|_| {
                    // internal server error mapping to S::Error is hard; for brevity return unauthorized response
                    ()
                })
                .ok();

            if let Some(mut conn) = conn {
                let black: Option<String> = conn.get(blacklist_key(&claims.jti)).await.ok();
                if black.is_some() {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body("Token blacklisted".into())
                        .unwrap());
                }
                let session_exists: Option<i64> = conn.get(session_key(&claims.jti)).await.ok();
                if session_exists.is_none() {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body("Session expired".into())
                        .unwrap());
                }

                // optional: check permissions cache in redis
                // For route required permission, we expect req.extensions().get::<RequiredPermission>()
                if let Some(required) = req
                    .extensions()
                    .get::<crate::auth::handlers::RequiredPermission>()
                {
                    // try cache
                    let perm_key = user_permissions_key(claims.sub);
                    let perms_cached: Option<String> = conn.get(&perm_key).await.ok();
                    let perms = if let Some(p) = perms_cached {
                        // parse as JSON array of strings
                        serde_json::from_str::<Vec<String>>(&p).unwrap_or_default()
                    } else {
                        // load from DB
                        let perms = get_permissions_for_user(&state.db, claims.sub)
                            .await
                            .unwrap_or_default();
                        // cache them
                        let _: () = conn
                            .set_ex(&perm_key, serde_json::to_string(&perms).unwrap(), 60 * 5)
                            .await
                            .unwrap_or(());
                        perms
                    };
                    if !perms.contains(&required.0.to_string()) {
                        return Ok(Response::builder()
                            .status(StatusCode::FORBIDDEN)
                            .body("Permission denied".into())
                            .unwrap());
                    }
                }
            } else {
                // if no redis access, fall back to DB check (simpler)
                if let Some(required) = req
                    .extensions()
                    .get::<crate::auth::handlers::RequiredPermission>()
                {
                    let perms = get_permissions_for_user(&state.db, claims.sub)
                        .await
                        .unwrap_or_default();
                    if !perms.contains(&required.0.to_string()) {
                        return Ok(Response::builder()
                            .status(StatusCode::FORBIDDEN)
                            .body("Permission denied".into())
                            .unwrap());
                    }
                }
            }

            // attach user id into extensions for handlers
            req.extensions_mut().insert(claims.sub);

            inner.call(req).await
        })
    }
}
