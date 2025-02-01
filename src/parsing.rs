//! HTTP request parsing logic


use std::{error::Error, fmt};

use crate::http::{Body, Headers, Method, Path, Query, Request, Sanitize};


#[derive(Debug)]
pub enum ParseError {
    //TODO: Complete this list while implementing parser
    InvalidMethod,
    MissingPath
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
fn parse_method(r: &str) -> Result<Method, ParseError> {

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


/// Parses the path from an HTTP/1.1 request
fn parse_path(r: &str) -> Result<Path, ParseError> {
    // First, split the request at each whitespace and collect into a vector,
    // then safely get the path (at index 1) and return the String if it was found.
    Ok(r.split(" ").collect::<Vec<&str>>().get(1)
        .ok_or(ParseError::MissingPath)?.to_owned().to_string().sanitize())
}



fn parse_headers(r: &str) -> Result<Headers, ParseError> {
    todo!();
}

fn parse_query(r: &str) -> Result<Query, ParseError> {
    todo!();
}


fn parse_body(r: &str) -> Result<Body, ParseError> {
    todo!();
}


/// Parses a UTF-8 String containing an HTTP request from a TCP stream into 
/// an internal representation `Request` type.
pub fn parse_http_request(data: &str) -> Result<Request, ParseError> {
    
    let method = parse_method(data)?;
    let path = parse_path(data)?;
    let query = parse_query(data)?;
    //let version = parse_version(data)?;
    let headers = parse_headers(data)?;


    let parsed_request = match method {
        Method::GET => Request::Get(path, query, headers),
        Method::POST => Request::Post(path, query, headers, parse_body(data)?),
        //Method::HEAD:: 
        //TODO: Finish implementing Request type and this match statement
    };


    todo!();
}




