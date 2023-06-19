use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("Configuration file is not found in `{path}`.")]
    #[diagnostic(
        code(gelatyx::no_config),
        url(docsrs),
        help("Try creating a config of your choosen formatter.")
    )]
    ConfigNotFound { path: PathBuf },

    #[error("Invalid configuration: {message}")]
    #[diagnostic(
        code(gelatyx::invalid_config),
        url(docsrs),
        help("See the configuration example of your choosen formatter.")
    )]
    InvalidConfig { message: String },
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<time::error::Format> for Error {
    fn from(err: time::error::Format) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<time::error::InvalidFormatDescription> for Error {
    fn from(err: time::error::InvalidFormatDescription) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<time::error::IndeterminateOffset> for Error {
    fn from(err: time::error::IndeterminateOffset) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<time::error::Parse> for Error {
    fn from(err: time::error::Parse) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<humantime::DurationError> for Error {
    fn from(err: humantime::DurationError) -> Self {
        Error::Internal(err.to_string())
    }
}
