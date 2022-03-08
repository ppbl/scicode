use actix_utils::future::{err, ok, Ready};
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header::AUTHORIZATION, Error as ActixError,
    FromRequest, HttpRequest,
};
use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub userid: i32,
    pub username: String,
    pub exp: usize,
}

static TOKEN_SECREN: Lazy<String> =
    Lazy::new(|| env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set"));

/** Get claims by authorization header */
pub fn get_claims(authorization: &str) -> Result<TokenData<Claims>, Error> {
    let token = authorization.split(" ").collect::<Vec<&str>>();
    decode::<Claims>(
        token[1],
        &DecodingKey::from_secret(TOKEN_SECREN.as_bytes()),
        &Validation::default(),
    )
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

impl FromRequest for Claims {
    type Error = ActixError;
    type Future = Ready<Result<Self, ActixError>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req.headers().get(AUTHORIZATION);
        if let Some(token) = token {
            let claims = get_claims(token.to_str().unwrap());
            match claims {
                Ok(TokenData { header: _, claims }) => ok(claims),
                Err(error) => err(ErrorUnauthorized(error)),
            }
        } else {
            err(ErrorUnauthorized("Please sign in"))
        }
    }
}
