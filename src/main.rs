#[macro_use]
extern crate rocket;

use diesel::PgConnection;
use dotenv::dotenv;
use rocket::figment::value::{Map, Value};
use rocket::figment::{map, Figment};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use rust_web_api_template::{get_api_catchers, get_auth_routes, ApiError, DbConn, User};
use std::env;

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
    let figment = rocket_figment();
    let mut rocket_builder = rocket::custom(figment);
    rocket_builder = rocket_builder
        .mount("/", routes![read])
        .mount("/", routes![edit])
        .mount("/api/security/v1", get_auth_routes())
        .attach(DbConnection::init())
        .register("/", get_api_catchers());

    rocket_builder
}

fn rocket_figment() -> Figment {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let db_pool_size = env::var("DATABASE_POOL_SIZE")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => db_pool_size.into()
    };
    rocket::Config::figment().merge(("databases", map!["db" => db]))
}
