use crate::request::header::Headers;

#[derive(Debug)]
pub struct Response {
  pub status: u32,
  headers: Headers,
  pub body: String
}

impl Response {
  pub fn new() -> Response {
    Response {
      status: 200,
      headers: Headers::new(),
      body: String::new()
    }
  }

  pub fn set_status(&mut self, status: u32) -> &Self {
    self.status = status;
    self
  }
}