use std::collections::HashMap;


pub trait Response {

}


pub type Path = String;
pub type Query = HashMap<String, String>;
pub type Headers = HashMap<String, String>;
pub type Body = String;


#[derive(Eq, Hash, PartialEq)]
pub enum Method {
    GET,
    POST
}

pub enum StatusCode {
    Ok,
    NotFound,
    //TODO
}

pub enum Request {
    Get(Path, Query, Headers),
    Post(Path, Query, Headers, Body)
}

