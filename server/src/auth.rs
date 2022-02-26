use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub userid: i32,
    pub username: String,
    pub exp: usize,
}

lazy_static! {
    static ref TOKEN_SECREN: String = env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");
}

/** Get claims by authorization header */
pub fn get_claims(authorization: &str) -> Claims {
    let token = authorization.split(" ").collect::<Vec<&str>>();
    decode::<Claims>(
        token[1],
        &DecodingKey::from_secret(TOKEN_SECREN.as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims
}

/** Generate token using claims */
pub fn generate_token(claims: Claims) -> String {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(TOKEN_SECREN.as_bytes()),
    )
    .unwrap()
}
