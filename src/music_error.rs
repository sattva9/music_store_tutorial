use actix_web::ResponseError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct MusicError {
    msg: String,
}

impl From<String> for MusicError {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for MusicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Error while processing Request: {}", self.msg)
    }
}

impl ResponseError for MusicError {}
