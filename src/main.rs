#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::{Build, Rocket};
use rust_web_api_template::{ApiError, User};

#[get("/")]
fn index(user: User) -> Result<&'static str, ApiError> {
    if user.has_action("HELLO/READ") {
        return Err(ApiError::from_status(Status::Unauthorized));
    }
    Ok("Hello, world!")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
