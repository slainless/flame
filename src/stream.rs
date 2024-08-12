use std::{io::{BufRead, BufReader}, net::TcpStream, error::Error as StdError};
use crate::request::{Location, Method, Request};

pub mod error;
use error::Error as ParseError;
type AnyError = Box<dyn StdError>;

const BUFFER_SIZE: usize = 4096;

fn parse_location(buf: &Vec<u8>) -> Result<Location, AnyError> {
  let mut location = Location(Method::Get, String::new());

  let str = String::from_utf8(buf.clone())?;
  let str: Vec<&str> = str.split(' ').collect();

  if str.len() != 3 {
    return Err(Box::new(ParseError::InvalidLocationFormat));
  }


  match str[0].to_lowercase().as_str() {
    "get" => location.0 = Method::Get,
    "post" => location.0 = Method::Post,
    method => {
      return Err(Box::new(ParseError::UnsupportedMethod(method.to_string())))
    }
  }
  
  let protocol = str[2].trim().to_lowercase();
  if protocol != "http/1.1" {
    return Err(Box::new(ParseError::UnsupportedProtocol(protocol)));
  }

  location.1 = str[1].to_string();
  Ok(location)
}

fn parse_header(buf: &Vec<u8>) -> Result<(String, String), Box<dyn StdError>> {
  let str = String::from_utf8(buf.clone())?;
  let str = str.split_once(':');

  if let Some((key, value)) = str {
    Ok((key.to_lowercase(), value.trim().to_lowercase()))
  } else {
    Err(Box::new(ParseError::InvalidHeaderEntryFormat))
  }
}

pub fn parse_stream(stream: TcpStream) -> Result<Request, AnyError> {
  let mut buf_reader = BufReader::new(stream);

  let mut request = Request::new();

  let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
  let mut line = 0;
  let mut header_sizes = 0;
  loop {
    let res = buf_reader.read_until(0xA, &mut buf)?;
    if header_sizes + res > BUFFER_SIZE {
      return Err(Box::new(ParseError::HeaderTooLong(BUFFER_SIZE)))
    }

    header_sizes += res;
    line += 1;

    if res == 0 {
      // header reading stops here...
      // caused by EOF
      break;
    }

    if 
      (res == 1 && buf[0] == 0xA) ||
      (res == 2 && buf[0] == 0xD && buf[1] == 0xA) 
    {
      // header reading stops here...
      // caused by incoming payload stream
      request.set_body(buf_reader);
      break;
    }

    if line == 1 {
      request.set_location(parse_location(&buf)?);
      buf.clear();
      continue;
    }

  
    let (key, value) = parse_header(&buf)?;
    request.headers().append(key, value);
    buf.clear();
  }

  if request.location().1 == "" {
    return Err(Box::new(ParseError::EmptyRequest))
  }
  return Ok(request)
}