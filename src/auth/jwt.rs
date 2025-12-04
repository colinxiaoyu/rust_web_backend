use anyhow::Result;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
}

pub fn make_claims(user_id: i64, expires_secs: i64) -> Claims {
    let now = Utc::now().timestamp();
    Claims {
        sub: user_id,
        iat: now,
        exp: now + expires_secs,
        jti: Uuid::new_v4().to_string(),
    }
}

pub fn encode_claims(secret: &[u8], claims: &Claims) -> Result<String> {
    let token = encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret),
    )?;
    Ok(token)
}

pub fn decode_claims(secret: &[u8], token: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(data.claims)
}
