//! HTTP request parsing logic


use std::{error::Error, fmt, collections::HashMap};

use crate::http::{Body, Headers, Method, Path, Query, Request, Sanitize};


#[derive(Debug)]
pub enum ParseError {
    //TODO: Complete this list while implementing parser
    InvalidMethod,
    MissingPath,
    MissingQuery,
    MalformedQuery,
    MissingStartLine,
    MissingBody,
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
    Ok(
        match e.split_once("?") {
            None => e,
            Some((p,_)) => p
        }.to_string().sanitize()
    )
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

/// Parses all headers from an HTTP message
fn parse_headers(r: &str) -> Result<Headers, ParseError> {
    Ok(Headers::from_iter(
    r.lines().skip(1) // Skip start line
        .fold(Vec::new(), |mut b, l| {
            if b.last().map_or("", |prev: &String| prev.trim()) != "" {
                // If the last line checked was non-empty, add current line and continue parsing
                // (This prevents the iterator from gaining any lines after the message metadata)
                b.push(l.to_string()); 
            }
            b
        })
        .into_iter()
        .map(|header_str| {
            // Parsing header strings into key value pairs
            let (key, val) = header_str.split_once(":")?;
            Some((key.to_string(), val.to_string()))
        })
        .filter(|maybe_kv| maybe_kv.is_some())
        .map(|some_kv| some_kv.unwrap())
    ))
}

/// Optionally parses a message body from an HTTP message
fn parse_body(r: &str) -> Result<Option<Body>, ParseError> {
    let b = r.split_once("\r\n\r\n").ok_or(ParseError::MissingBody)?.1;
    if b == "" {
        // No body
        return Ok(None)
    }

    Ok(Some(b.to_string()))
}


/// Parses a UTF-8 String containing an HTTP request from a TCP stream into 
/// an internal representation `Request` type.
pub fn parse_http_request(data: &str) -> Result<Request, ParseError> {
    
    let method = parse_method(data)?;
    eprintln!("Parsed method!: {:?}", method);
    let endpoint = parse_endpoint(data)?;
    eprintln!("Parsed endpoint!: {:?}", endpoint);
    let path = parse_path(&endpoint)?;
    eprintln!("Parsed path!: {:?}", path);
    
    let query = parse_query(data)?;
    //let version = parse_version(data)?;
    let headers = parse_headers(data)?;
    let body = parse_body(data)?;


    Ok(Request::from_parts(method,path,query,headers,body))
}




