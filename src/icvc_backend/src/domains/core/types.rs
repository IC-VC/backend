use std::fmt;

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub enum APIError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    InternalServerError(String),
    MultipleErrors(Vec<APIError>),
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            APIError::BadRequest(msg) => write!(f, "BadRequest: {}", msg),
            APIError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            APIError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            APIError::NotFound(msg) => write!(f, "NotFound: {}", msg),
            APIError::InternalServerError(msg) => write!(f, "InternalServerError: {}", msg),
            APIError::MultipleErrors(errors) => {
                write!(f, "MultipleErrors: [")?;
                for (i, error) in errors.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", error)?;
                }
                write!(f, "]")
            }
        }
    }
}
