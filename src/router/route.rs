use std::fmt::Display;
use std::fmt;

#[derive(PartialEq, Hash, Copy, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

impl Eq for Method {}
impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let method = match self {
            Method::GET => {
                "GET"
            },
            Method::POST => {
                "POST"
            },
            Method::PUT => {
                "PUT"
            },
            Method::DELETE => {
                "DELETE"
            },
        };

        write!(f, "{}", method)
    }
}

pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: Box<Fn() + Send + 'static>,
}

impl Route {
    pub fn new(path: String, method: Method, handler: Box<Fn() + Send + 'static>) -> Self {
        Route {path, method, handler}
    }
}