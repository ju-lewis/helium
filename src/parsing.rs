//! HTTP request parsing logic


use crate::http::Method;

/// Parses the method from an HTTP/1.1 request
pub fn parse_method(r: &str) -> Option<Method> {
    
    let method_str = r.split_once(" ")?.0;
        
    match method_str {
        "GET"     => Some(Method::GET),
        "POST"    => Some(Method::POST),
        "PUT"     => Some(Method::PUT),
        "DELETE"  => Some(Method::DELETE),
        "PATCH"   => Some(Method::PATCH),
        "HEAD"    => Some(Method::HEAD),
        "OPTIONS" => Some(Method::OPTIONS),
        "CONNECT" => Some(Method::CONNECT),
        "TRACE"   => Some(Method::TRACE),
        _ => None,
    }
}


