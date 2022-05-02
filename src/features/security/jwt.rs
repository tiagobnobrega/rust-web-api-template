use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::ApiError;
use jsonwebtoken::errors::{Error as JwtError, ErrorKind, Result as JwtResult};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TokenType {
    DEFAULT,
    EMAIL,
    RESET,
}

pub struct TokenTypeConfig {
    duration: usize,
    iss: String,
}

// TODO: Parametrize this
const TOKEN_ISS: &str = "rust-web-api-template";
const JWT_KEY: &[u8] = b"changeme";
const DECODING_KEY: DecodingKey = DecodingKey::from_secret(JWT_KEY);
const ENCODING_KEY:EncodingKey = EncodingKey::from_secret(JWT_KEY);

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
    pub exp: usize,
    pub sub: String,
    pub aud: TokenType,
}

impl TryFrom<String> for Claims {
    type Error = ApiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let token_data = match decode::<Claims>(&value, , &validation) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"),
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => ,
            },
        };
    }
}

impl Claims {
    pub fn toJwt(&self) -> JwtResult<String> {
        encode(
            &Header::default(),
            &self,
            &ENCODING_KEY,
        )
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = ApiError<'r>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        let auth_opt = headers.get_one("Authorization");
        match auth_opt {
            None => Outcome::Forward(()),
            Some(authorization) => {
                let token = match encode(
                    &Header::default(),
                    &my_claims,
                    &EncodingKey::from_secret(key),
                ) {
                    Ok(t) => t,
                    Err(_) => panic!(), // in practice you would return the error
                };
            }
        }
    }
}
