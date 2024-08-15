use std::{collections::HashMap, error::Error, net::TcpListener};

use crate::{request::Request, router::Router, stream};

pub struct App {
  router: Router
}

impl App {
  pub fn new() -> App {
    App{
      router: Router::new()
    }
  }

  // pub fn get(&self, path: &str, handler: Box<dyn Fn(Request) -> ()>) {
  //   self.handlers
  //     .entry(path.to_string())
  //     .or_insert(Vec::new())
  //     .push(handler);
  // }

  pub fn listen(&self, address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;

    for incoming in listener.incoming() {
      if incoming.is_err() {
        continue
      }

      let mut req = match stream::parse_stream(incoming.unwrap()) {
        Ok(req) => req,
        Err(err) => {
          println!("Error parsing stream: {err:?}");
          continue;
        }
      };

      // let handler = match self.determine_handler(&req) {
      //   Ok(handler) => handler,
      //   Err(err) => {
      //     continue
      //   }
      // };

      // handler(req);
    }

    return Ok(())
  }
}