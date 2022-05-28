use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::{Catcher, Request};

use crate::ApiError;

#[catch(default)]
pub fn handle_api_error(status: Status, req: &Request) -> Custom<ApiError> {
    Custom(
        status,
        ApiError::get_request_cached_or_default(req, status).clone(),
    )
}

pub fn get_api_catchers() -> Vec<Catcher> {
    catchers![handle_api_error]
}
