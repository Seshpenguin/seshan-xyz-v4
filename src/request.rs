use rust_cgi as cgi;
use rust_cgi::http::{HeaderValue, header};

use crate::error::{Error, Result};

pub trait RequestExt {
    fn header_str(&self, name: &str) -> Option<&str>;
}

impl RequestExt for cgi::Request {
    fn header_str(&self, name: &str) -> Option<&str> {
        self.headers().get(name)?.to_str().ok()
    }
}

pub fn redirect(location: &str) -> Result<cgi::Response> {
    let location = HeaderValue::from_str(location).map_err(|_| Error::BadRequest)?;
    let mut response = cgi::empty_response(301);
    response.headers_mut().insert(header::LOCATION, location);
    Ok(response)
}
