use std::{collections::HashMap, error::Error, fmt};


pub trait Response {

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

#[derive(Debug)]
pub enum ParseError {
    //TODO: Complete this list while implementing parser
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "A parsing error occurred.")
        }
    }
}
impl Error for ParseError {}


/// Parses a UTF-8 String containing an HTTP request from a TCP stream into 
/// an internal representation `Request` type.
pub fn parse_http_request(data: &str) -> Result<Request, ParseError> {
    
    
    
    todo!();
}

/// Creates a formatted HTTP response
pub fn create_response() -> String {
    todo!();
}
