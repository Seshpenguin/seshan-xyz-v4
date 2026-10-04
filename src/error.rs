use std::io;

use rust_cgi as cgi;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    BadRequest,
    NotFound,
    Internal(String),
}

impl Error {
    pub fn into_response(self) -> cgi::Response {
        match self {
            Error::BadRequest => cgi::empty_response(400),
            Error::NotFound => cgi::empty_response(404),
            Error::Internal(message) => {
                eprintln!("{message}");
                cgi::empty_response(500)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => Error::NotFound,
            _ => Error::Internal(err.to_string()),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Internal(format!("invalid site config: {err}"))
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        Error::Internal(err.to_string())
    }
}
