use regex::{Error, Regex};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

use crate::features::security::jwt::Claims;
use crate::features::shared::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRole<'r> {
    name: &'r str,
    actions: Vec<&'r str>,
}

impl<'r> UserRole<'r> {
    pub fn new(name: &'r str, actions: Vec<&'r str>) -> Self {
        Self { name, actions }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User<'r> {
    username: String,
    #[serde(borrow)]
    pub roles: Vec<UserRole<'r>>,
}
impl<'r> User<'r> {
    pub fn new(username: String) -> Self {
        Self {
            username,
            roles: Vec::new(),
        }
    }

    fn get_action_list(&self) -> Vec<&str> {
        self.roles
            .iter()
            .flat_map(|role| role.actions.to_vec())
            .collect::<Vec<&str>>()
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
impl<'r> FromRequest<'r> for User<'r> {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let claims_outcome = request.guard::<Claims>().await;

        match claims_outcome {
            Outcome::Success(claims) => match claims.sub.as_str() {
                "user" => {
                    let mut user = User::new("user".to_string());
                    user.roles.push(UserRole::new("ROLE_A", vec!["HELLO/READ"]));
                    Outcome::Success(user)
                }
                "admin" => {
                    let mut user = User::new("admin".to_string());
                    user.roles
                        .push(UserRole::new("ROLE_A", vec!["HELLO/READ", "HELLO/EDIT"]));
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
