#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rust_web_api_template::{get_api_catchers, get_auth_routes, ApiError, User};

#[get("/read")]
fn read(user_res: Result<User, ApiError>) -> Result<Json<User>, ApiError> {
    let user = user_res?;
    dbg!(&user);
    if !user.has_action("HELLO/READ") {
        return Err(ApiError::from_status(Status::Forbidden));
    }

    Ok(Json(user))
}

#[get("/edit")]
fn edit(user_res: Result<User, ApiError>) -> Result<Json<User>, ApiError> {
    let user = user_res?;
    dbg!(&user);
    if !user.has_action("HELLO/EDIT") {
        return Err(ApiError::from_status(Status::Forbidden));
    }
    Ok(Json(user))
}

#[launch]
fn rocket() -> Rocket<Build> {
    let mut rocket_builder = rocket::build();
    rocket_builder = rocket_builder.mount("/", routes![read]);
    rocket_builder = rocket_builder.mount("/", routes![edit]);
    rocket_builder = rocket_builder.mount("/api/security/v1", get_auth_routes());
    rocket_builder = rocket_builder.register("/", get_api_catchers());
    rocket_builder
}
