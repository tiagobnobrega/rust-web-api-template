use crate::ApiError;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginData {
    user: Option<String>,
    password: Option<String>,
}

#[post("/login", format = "json", data = "<login_data>")]
pub async fn login(login_data: Json<LoginData>) -> Result<String, ApiError> {
    if login_data.user.is_none() {
        return Err(ApiError::from_status_message(
            Status::BadRequest,
            "Argument user must be defined",
        ));
    }
    if login_data.password.is_none() {
        return Err(ApiError::from_status_message(
            Status::BadRequest,
            "Argument password must be defined",
        ));
    }
    Ok("OK".to_string())
}
