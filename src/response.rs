use std::{error::Error, io::{BufWriter, Write}, net::TcpStream};

use crate::{header::Headers, protocol::{self, StatusCode}, Return};

#[derive(Debug)]
pub struct Response {
  pub status: StatusCode,
  pub headers: Headers,
  stream: Option<BufWriter<TcpStream>>,

  is_header_written: bool,
  is_body_written: bool
}

impl Response {
  pub(crate) fn new(stream: Option<BufWriter<TcpStream>>) -> Response {
    Response {
      status: StatusCode::OK,
      headers: Headers::new(),
      stream: stream,

      is_header_written: false,
      is_body_written: false
    }
  }

  pub fn status(&mut self, status: StatusCode) -> &mut Self {
    self.status = status;
    self
  }

  pub fn content_type(&mut self, content_type: &str) -> &mut Self {
    self.headers.set("Content-Type", content_type.to_string());
    self
  }

  pub fn send_headers(&mut self) -> Result<Return, Box<dyn Error>> {
    if self.is_header_written {
      return Ok(Return::End);
    }

    let writer = self.stream.as_mut().unwrap();
    writer.write_fmt(format_args!("{} {} {}\r\n", 
      protocol::HTTP_PROTOCOL, 
      self.status.to_string(), 
      self.status.reason_phrase())
    )?;

    for (key, value) in self.headers.iter() {
      for value in value.iter() {
        writer.write_fmt(format_args!("{}: {}\r\n", key, value))?;
      }
    }

    self.is_header_written = true;
    Ok(Return::End)
  }

  pub fn send_body(&mut self, body: Vec<u8>) -> Result<Return, Box<dyn Error>> {
    if self.is_body_written {
      return Ok(Return::End);
    }

    self.headers.set("Content-Length", body.len().to_string());
    self.send_headers()?;

    let writer = self.stream.as_mut().unwrap();
    writer.write(b"\r\n")?;
    writer.write_all(&body)?;
    writer.flush()?;

    self.is_body_written = true;
    Ok(Return::End)
  }
}