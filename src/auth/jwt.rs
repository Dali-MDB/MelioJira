use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}


pub fn generate_token(
    payload: &Claim,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claim, jsonwebtoken::errors::Error> {
    let payload = decode::<Claim>(
        token,
        &DecodingKey::from_secret((secret).as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(payload.claims)
}
