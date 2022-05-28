use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Route;

use crate::features::security::jwt::{Claims, TokenType};
use crate::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicAuthData {
    user_id: Option<String>,
    password: Option<String>,
}

#[post("/auth/basic", format = "json", data = "<basic_auth_data>")]
pub fn basic_authenticate(basic_auth_data: Json<BasicAuthData>) -> Result<String, ApiError> {
    let user_id = basic_auth_data.user_id.as_ref().ok_or_else(|| {
        ApiError::from_status_message(Status::BadRequest, "Argument user must be defined")
    })?;
    let password = basic_auth_data.password.as_ref().ok_or_else(|| {
        ApiError::from_status_message(Status::BadRequest, "Argument password must be defined")
    })?;

    match user_id.as_str() {
        "user" => {
            if password != "123456" {
                return Err(ApiError::from_status(Status::Forbidden));
            }
            let claims = Claims::new("user".to_string(), TokenType::DEFAULT);
            Ok(claims.toJwt()?)
        }
        "admin" => {
            if password != "123456" {
                return Err(ApiError::from_status(Status::Forbidden));
            }
            let claims = Claims::new("admin".to_string(), TokenType::DEFAULT);
            Ok(claims.toJwt()?)
        }
        _ => Err(ApiError::from_status(Status::Forbidden)),
    }
}

pub fn get_auth_routes() -> Vec<Route> {
    return routes![basic_authenticate];
}
