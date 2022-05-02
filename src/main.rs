#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::{Build, Rocket};
use rocket_contrib::json::Json;
use rust_web_api_template::{ApiError, User};

#[get("/read")]
fn read(user: User) -> Result<&'static str, ApiError> {
    dbg!(&user);
    if !user.has_action("HELLO/READ") {
        return Err(ApiError::from_status(Status::Forbidden));
    }
    Ok("Hello, world!")
}

#[get("/edit")]
fn edit(user: User) -> Result<&'static str, ApiError> {
    dbg!(&user);
    if !user.has_action("HELLO/EDIT") {
        return Err(ApiError::from_status(Status::Forbidden));
    }
    Ok("Hello, world!")
}

#[launch]
fn rocket() -> Rocket<Build> {
    let mut rocket_builder = rocket::build();
    rocket_builder = rocket_builder.mount("/", routes![read]);
    rocket_builder = rocket_builder.mount("/", routes![edit]);
    rocket_builder
}
