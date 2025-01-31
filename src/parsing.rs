//! HTTP request parsing logic


use std::{error::Error, fmt};

use crate::http::{Method, Request};


#[derive(Debug)]
pub enum ParseError {
    //TODO: Complete this list while implementing parser
    InvalidMethod
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "A parsing error occurred.")
        }
    }
}
impl Error for ParseError {}




/// Parses the method from an HTTP/1.1 request
pub fn parse_method(r: &str) -> Result<Method, ParseError> {
    

    let method_str = r.split_once(" ").ok_or(ParseError::InvalidMethod)?.0;

    match method_str {
        "GET"     => Ok(Method::GET),
        "POST"    => Ok(Method::POST),
        "PUT"     => Ok(Method::PUT),
        "DELETE"  => Ok(Method::DELETE),
        "PATCH"   => Ok(Method::PATCH),
        "HEAD"    => Ok(Method::HEAD),
        "OPTIONS" => Ok(Method::OPTIONS),
        "CONNECT" => Ok(Method::CONNECT),
        "TRACE"   => Ok(Method::TRACE),
        _ => Err(ParseError::InvalidMethod),
    }
}





/// Parses a UTF-8 String containing an HTTP request from a TCP stream into 
/// an internal representation `Request` type.
pub fn parse_http_request(data: &str) -> Result<Request, ParseError> {
    
    let method = parse_method(data)?;
    
    

    todo!();
}




