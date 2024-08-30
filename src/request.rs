use std::{collections::HashMap, io::BufReader, net::TcpStream};

use crate::header::Headers;

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
  pub(crate) fn new(reader: Option<BufReader<TcpStream>>) -> Request {
    Request {
      headers: Headers::new(),
      location: Location(Method::Get, String::new()),
      body: reader
    }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }

  pub fn headers(&self) -> &Headers {
    &self.headers
  }

  pub fn body(&mut self) -> &Option<BufReader<TcpStream>> {
    &mut self.body
  }

  pub(crate) fn set_body(&mut self, body: Option<BufReader<TcpStream>>) {
    self.body = body
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
}
