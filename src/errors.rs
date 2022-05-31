use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvloadError {
    #[error("environment variable not found")]
    EnvVarNotFound,
    #[error("parse error")]
    ParseError,
    #[error("env variable is invalid unicode data")]
    InvalidUnicodeData,
}
