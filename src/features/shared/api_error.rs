use jsonwebtoken::errors::{Error as JwtError, ErrorKind, Result as JwtResult};
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
pub struct ApiError {
    http_status: u16,
    code: String,
    message: String,
}

impl ApiError {
    pub fn new(http_status: Status, code: &str, message: &str) -> Self {
        Self {
            http_status: http_status.code,
            code: code.to_string(),
            message: message.to_string(),
        }
    }
    pub fn from_status_code(http_status: Status, code: &str) -> Self {
        Self {
            http_status: http_status.code,
            code: code.to_string(),
            message: http_status
                .reason()
                .unwrap_or("Unexpected Error")
                .to_string(),
        }
    }
    pub fn from_status(http_status: Status) -> Self {
        Self {
            http_status: http_status.code,
            code: format!("000-{}", http_status.code),
            message: http_status
                .reason()
                .unwrap_or("Unexpected Error")
                .to_string(),
        }
    }
    pub fn from_status_message(http_status: Status, message: &str) -> Self {
        Self {
            http_status: http_status.code,
            code: format!("000-{}", http_status.code),
            message: message.to_string(),
        }
    }

    pub fn to_outcome_failure(self) -> (Status, Self) {
        (
            Status::from_code(self.http_status).unwrap_or(Status::InternalServerError),
            self,
        )
    }
    //? TODO: Create a new "delegated" type for the local_cache ?
    pub fn to_outcome_failure_cached(self, request: &Request) -> (Status, Self) {
        request.local_cache(|| self.clone());
        (
            Status::from_code(self.http_status).unwrap_or(Status::InternalServerError),
            self,
        )
    }
    pub fn get_request_cached_or_default<'r>(request: &'r Request<'_>, status: Status) -> &'r Self {
        request.local_cache(|| Self::from_status(status))
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
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

impl From<JwtError> for ApiError {
    fn from(err: JwtError) -> Self {
        let base_message = "The token is invalid";
        let code = format!("jwt-{}", &err);
        let message = format!("{} ({})", base_message, &err);
        let status = match *err.kind() {
            ErrorKind::InvalidToken
            | ErrorKind::InvalidKeyFormat
            | ErrorKind::MissingRequiredClaim(_)
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::MissingAlgorithm
            | ErrorKind::Base64(_)
            | ErrorKind::Json(_)
            | ErrorKind::Utf8(_)
            | ErrorKind::Crypto(_) => Status::BadRequest,
            ErrorKind::InvalidIssuer
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidEcdsaKey
            | ErrorKind::InvalidRsaKey(_)
            | ErrorKind::RsaFailedSigning
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::ImmatureSignature => Status::Forbidden,
            ErrorKind::InvalidAlgorithmName => Status::UnprocessableEntity,
            ErrorKind::ExpiredSignature => Status::Unauthorized,
            _ => Status::InternalServerError,
        };
        ApiError::new(status, code.as_str(), message.as_str())
    }
}
