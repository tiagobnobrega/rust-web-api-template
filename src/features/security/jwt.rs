use chrono::{Duration, Utc};
use std::fmt::Debug;

use jsonwebtoken::errors::Result as JwtResult;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

use crate::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub enum TokenType {
    DEFAULT,
    EMAIL,
    RESET,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenTypeConfig {
    duration: i64,
    iss: String,
}

// TODO: Parametrize this
const TOKEN_ISS: &str = "rust-web-api-template";
const JWT_KEY: &[u8] = b"changeme";

// TODO: Add these to app State or something
// static DECODING_KEY: DecodingKey = DecodingKey::from_secret(JWT_KEY);
// static ENCODING_KEY: EncodingKey = EncodingKey::from_secret(JWT_KEY);

impl TokenType {
    pub fn getConfig(&self) -> TokenTypeConfig {
        match &self {
            TokenType::DEFAULT => TokenTypeConfig {
                duration: 60_000 * 10,
                iss: TOKEN_ISS.to_string(),
            },
            TokenType::EMAIL => TokenTypeConfig {
                duration: 60_000 * 60 * 48,
                iss: TOKEN_ISS.to_string(),
            },
            TokenType::RESET => TokenTypeConfig {
                duration: 60_000 * 60 * 2,
                iss: TOKEN_ISS.to_string(),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
    pub aud: TokenType,
}

impl TryFrom<&str> for Claims {
    type Error = ApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        //TODO: improve token validation
        let validation = Validation::new(Algorithm::HS256);
        //TODO: Remove DecodingKey instantiation
        match decode::<Claims>(value, &DecodingKey::from_secret(JWT_KEY), &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(ApiError::from(err)),
        }
    }
}

impl Claims {
    pub fn toJwt(&self) -> JwtResult<String> {
        //TODO: Remove EncodingKey instantiation
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(JWT_KEY),
        )
    }
    pub fn new(sub: String, aud: TokenType) -> Self {
        let now = Utc::now();
        let iat = now.timestamp();
        let exp_dt = now + Duration::days(aud.getConfig().duration);
        let exp = exp_dt.timestamp();
        Self { exp, sub, aud, iat }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        let auth_opt = headers.get_one("Authorization");
        match auth_opt {
            None => Outcome::Forward(()),
            Some(authorization) => {
                let authorization_parts: Vec<&str> = authorization.split("Bearer ").collect();
                let token = *authorization_parts.get(1).unwrap_or(&"");
                match Claims::try_from(token) {
                    Ok(claims) => Outcome::Success(claims),
                    Err(err) => Outcome::Failure(err.to_outcome_failure()),
                }
            }
        }
    }
}
