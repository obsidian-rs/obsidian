use std::fmt::Display;
use std::fmt;
use hyper::Method;

use super::EndPointHandler;

pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: Box<EndPointHandler>,
}

impl Route {
    pub fn new(path: String, method: Method, handler: Box<EndPointHandler>) -> Self {
        Route {path, method, handler}
    }
}