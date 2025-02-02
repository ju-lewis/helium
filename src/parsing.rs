//! HTTP request parsing logic


use std::{error::Error, fmt, collections::HashMap};

use crate::http::{Body, Headers, Method, Path, Query, Request, Sanitize};


#[derive(Debug)]
pub enum ParseError {
    //TODO: Complete this list while implementing parser
    InvalidMethod,
    MissingPath,
    MissingQuery,
    MalformedQuery
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

/// Parses the endpoint (path and query string) from an HTTP/1.1 request
fn parse_endpoint(r: &str) -> Result<String, ParseError> {
    // First, split the request at each whitespace and collect into a vector,
    // then safely get the path (at index 1) and return the String if it was found.
    Ok(r.split(" ").collect::<Vec<&str>>().get(1)
        .ok_or(ParseError::MissingPath)?.to_owned().to_string())
}

/// Parses the path from an endpoint (path and query string)
fn parse_path(e: &str) -> Result<Path, ParseError> {
    Ok(e.split_once("?").ok_or(ParseError::MissingPath)?.0.to_string().sanitize())
}

// Utility function for mapping over query parameters
fn tuple_str_to_string(t: (&str, &str)) -> (String, String) {
    (String::from(t.0), String::from(t.1))
}


/// Parses the query string from an endpoint (path and query string)
fn parse_query(e: &str) -> Result<Query, ParseError> {
    let query_string = e.split_once("?").ok_or(ParseError::MissingQuery)?.1.to_string();

    let mut params = query_string.split("&").map(|kv| {
        // kv is a string representation of a key-value pair

        let (key, value) = kv.split_once("=").ok_or(ParseError::MalformedQuery)?;

        Ok::<(&str, &str), ParseError>((key, value))
    });


    if params.any(|x| x.is_err()) {
        return Err(ParseError::MalformedQuery);
    }


    // Wow that's a lot of nested brackets lol
    Ok(HashMap::from_iter(params.map(|x| tuple_str_to_string(x.expect("It has been verified all elements of this iterator are of Ok(x)")))))
}

fn parse_headers(r: &str) -> Result<Headers, ParseError> {
    todo!();
}

fn parse_body(r: &str) -> Result<Option<Body>, ParseError> {
    todo!();
}


/// Parses a UTF-8 String containing an HTTP request from a TCP stream into 
/// an internal representation `Request` type.
pub fn parse_http_request(data: &str) -> Result<Request, ParseError> {
    
    let method = parse_method(data)?;
    let endpoint = parse_endpoint(data)?;
    let path = parse_path(&endpoint)?;
    
    let query = parse_query(data)?;
    //let version = parse_version(data)?;
    let headers = parse_headers(data)?;
    let body = parse_body(data)?;



    Ok(Request::from_parts(method,path,query,headers,body))
}




