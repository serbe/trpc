use std::result;

use thiserror::Error as ThisError;

pub type Result<T> = result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("NC error")]
    NC(#[from] nc::error::Error),
    #[error("json error")]
    JSON(#[from] serde_json::Error),
    #[error("dotenv error")]
    DotEnv(#[from] dotenv::Error),
    #[error("response not success: {0}")]
    BadResponse(String),
    #[error("response no contain arguments")]
    NoArguments,
    #[error("unmutable fields in session-set")]
    WrongSessionSetFields,
    #[error("io error")]
    IO(#[from] std::io::Error),
}
