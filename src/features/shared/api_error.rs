use rocket::http::{ContentType, Header, Status};
use rocket::response::{Responder, Response, Result as ResponseResult};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

// #[derive(Responder)]
// pub struct ApiErrorResponder<'a> {
//     inner: String,
//     code: Header<'a>,
// }
//
// impl<'a> ApiErrorResponder<'a> {
//     pub fn new(inner: String, code: String) -> Self {
//         Self {
//             inner,
//             code: Header::new("api-error-code", code),
//         }
//     }
// }
//
// impl<'a> TryFrom<ApiError<'a>> for ApiErrorResponder<'a> {
//     type Error = serde_json::Error;
//
//     fn try_from(api_error: ApiError) -> Result<Self, Self::Error> {
//         let json = serde_json::to_string(&api_error)?;
//         Ok(ApiErrorResponder::new(json, api_error.code))
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError<'a> {
    http_status: u16,
    code: String,
    message: &'a str,
}

impl<'a> ApiError<'a> {
    pub fn new(http_status: Status, code: &str, message: &'a str) -> Self {
        Self {
            http_status: http_status.code,
            code: code.to_string(),
            message,
        }
    }
    pub fn from_status_code(http_status: Status, code: &str) -> Self {
        Self {
            http_status: http_status.code,
            code: code.to_string(),
            message: http_status.reason().unwrap_or("Unexpected Error"),
        }
    }
    pub fn from_status(http_status: Status) -> Self {
        Self {
            http_status: http_status.code,
            code: format!("000-{}", http_status.code),
            message: http_status.reason().unwrap_or("Unexpected Error"),
        }
    }
    pub fn to_outcome_failure(self) -> (Status, Self) {
        (
            Status::from_code(self.http_status).unwrap_or(Status::InternalServerError),
            self,
        )
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError<'o> {
    fn respond_to(self, _: &'r Request<'_>) -> ResponseResult<'o> {
        let json_res = serde_json::to_string(&self);
        match json_res {
            Ok(json) => Ok(Response::build()
                .sized_body(json.len(), Cursor::new(json))
                .header(ContentType::new("application", "json"))
                .status(Status::from_code(self.http_status).unwrap_or(Status::InternalServerError))
                .finalize()),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}
