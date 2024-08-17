use crate::header::{self, Headers};

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

pub fn merge_move(from: Response, to: &mut Response) {
  to.status = from.status;
  header::merge_move(from.headers, &mut to.headers);
  to.body = from.body;
}

pub fn merge_copy(from: &Response, to: &mut Response) {
  to.status = from.status;
  header::merge_copy(&from.headers, &mut to.headers);
  to.body = from.body.clone();
}