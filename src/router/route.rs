#[derive(PartialEq, Hash)]
pub enum Method {
  GET,
  POST,
  PUT,
  DELETE
}

impl Eq for Method {}

pub struct Route<'a> {
  pub path: String,
  pub handler: Box<Fn() + 'a>,
}

impl <'a> Route<'a> {
  pub fn new(path: String, handler: Box<Fn() + 'a>) -> Self {
    Route {path, handler}
  }
}