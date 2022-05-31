use diesel::{Associations, Identifiable, Queryable};
use regex::{Error, Regex};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use schema::{roles, users};
use serde::{Deserialize, Serialize};

use crate::features::security::jwt::Claims;
use crate::features::shared::ApiError;
use crate::schema;

#[derive(Queryable, Associations, Debug, Deserialize, Serialize)]
#[table_name = "roles"]
#[belongs_to(parent = User<'_>,foreign_key="username")]
pub struct Role<'r> {
    username: &'r str,
    name: &'r str,
    actions: Vec<&'r str>,
}

impl<'r> Role<'r> {
    pub fn new(username: &'r str, name: &'r str, actions: Vec<&'r str>) -> Self {
        Self {
            name,
            actions,
            username,
        }
    }
}

// Fields should be on the same order as defined in the schema.rs
#[derive(Identifiable, Insertable)]
#[table_name = "users"]
#[primary_key(username)]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub primary_email: &'a String,
    pub password: &'a String,
    pub first_name: &'a String,
    pub last_name: &'a String,
}

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    primary_email: String,
    password: String,
    first_name: String,
    last_name: String,
}
impl User {
    pub fn new(username: String) -> Self {
        Self {
            username,
            primary_email: "".to_string(),
            password: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
        }
    }

    fn get_action_list(&self) -> Vec<&str> {
        return vec!["Unimplemented"];
        // self.roles
        //     .iter()
        //     .flat_map(|role| role.actions.to_vec())
        //     .collect::<Vec<&str>>()
    }
    pub fn has_action(&self, action: &str) -> bool {
        self.get_action_list()
            .iter()
            .any(|&role_action| role_action == action)
    }
    pub fn has_any_action(&self, actions: Vec<&str>) -> bool {
        self.get_action_list()
            .iter()
            .any(|role_action| actions.iter().any(|action| role_action == action))
    }
    pub fn has_action_matching(&self, regex_pat: &str) -> Result<bool, Error> {
        let re = Regex::new(regex_pat)?;
        Ok(self
            .get_action_list()
            .iter()
            .any(|role_action| re.is_match(role_action)))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let claims_outcome = request.guard::<Claims>().await;

        match claims_outcome {
            Outcome::Success(claims) => match claims.sub.as_str() {
                "user" => {
                    let mut user = User::new("user".to_string());
                    Outcome::Success(user)
                }
                "admin" => {
                    let mut user = User::new("admin".to_string());
                    Outcome::Success(user)
                }
                &_ => {
                    Outcome::Failure(ApiError::from_status(Status::Forbidden).to_outcome_failure())
                }
            },
            Outcome::Failure(fail_data) => Outcome::Failure(fail_data),
            Outcome::Forward(forward_data) => Outcome::Forward(forward_data),
        }
    }
}
