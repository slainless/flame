use std::{collections::HashMap, net::TcpStream};

use header::Headers;

pub mod header;

#[derive(Debug)]
pub struct Request {
  location: Location,
  headers: Headers,
  body: Option<TcpStream>
}

#[derive(Debug)]
pub enum Method {
  Post,
  Get
}

#[derive(Debug)]
pub struct Location(
  pub Method, 
  pub URI
);

pub type Header = HashMap<String, Vec<String>>;
pub type URI = String;

impl Request {
  pub fn new() -> Request {
    Request {
      headers: Headers::new(),
      location: Location(Method::Get, String::new()),
      body: None
    }
  }

  pub fn set_location(&mut self, loc: Location) {
    self.location = loc
  }

  pub fn get_location(&self) -> &Location {
    &self.location
  }

  pub fn set_headers(&mut self, headers: Headers) {
    self.headers = headers
  }

  pub fn get_headers(&mut self) -> &mut Headers {
    &mut self.headers
  }

  pub fn set_body(&mut self, stream: TcpStream) {
    self.body = Some(stream)
  }
}
