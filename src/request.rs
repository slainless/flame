use std::{collections::HashMap, io::BufReader, net::TcpStream};

use crate::header::Headers;

#[derive(Debug)]
pub struct Request {
  location: Location,
  headers: Headers,
  body: Option<BufReader<TcpStream>>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Method {
  All,
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

  pub fn location(&self) -> &Location {
    &self.location
  }

  pub fn headers(&self) -> &Headers {
    &self.headers
  }

  pub(crate) fn set_location(&mut self, loc: Location) {
    self.location = loc
  }

  pub(crate) fn set_headers(&mut self, headers: Headers) {
    self.headers = headers
  }

  pub(crate) fn mut_headers(&mut self) -> &mut Headers {
    &mut self.headers
  }

  pub(crate) fn set_body(&mut self, stream: BufReader<TcpStream>) {
    self.body = Some(stream)
  }

  pub(crate) fn body(&mut self) -> &mut Option<BufReader<TcpStream>> {
    &mut self.body
  }
}
