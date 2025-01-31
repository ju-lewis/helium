

use std::{collections::HashMap, error::Error, fmt};
use crate::parsing;


pub trait Response {

}

pub trait Sanitize {
    /// Ensure no path-traversal unintended file access occurs
    fn sanitize(&self) -> Self;
}


pub type Path = String;
pub type Query = HashMap<String, String>;
pub type Headers = HashMap<String, String>;
pub type Body = String;


#[derive(Eq, Hash, PartialEq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    PATCH,
    TRACE
}

pub enum StatusCode {

    // Success codes
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,


    // Redirection codes
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    // 305 is deprecated
    // 306 is unused
    TemporaryRedirect = 307,
    PermanentRedirect = 308,


    // Client error codes
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    ContentTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    // 418 is unused
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    UpgradeRequired = 426,


    // Server error codes
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505
}

pub enum Request {
    Get(Path, Query, Headers),
    Post(Path, Query, Headers, Body)
}


impl Sanitize for Path {
    fn sanitize(&self) -> Self {

        // Remove any relative path prefixes (e.g. ../, / ./)
        let mut i = 0;
        for c in self.chars() {
            if !['.', '/'].contains(&c) {
                return match self.split_at_checked(i) {
                    None => self,
                    Some(s) => s.0
                }.to_string();
            }
            i += 1;
        }

        // This state is only reachable for empty paths (which are impossible to obtain)
        self.to_string()
    }
}


/// Creates a formatted HTTP response
pub fn create_response() -> String {
    todo!();
}



