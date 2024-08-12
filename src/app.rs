use std::{cell::RefCell, collections::HashMap, error::Error, net::TcpListener};

use crate::{request::Request, stream};

pub struct App {
  handlers: RefCell<HashMap<String, Vec<Box<dyn Fn(Request) -> ()>>>>
}

impl App {
  pub fn new() -> App {
    App{
      handlers: RefCell::new(HashMap::new())
    }
  }

  pub fn get(&self, path: &str, handler: Box<dyn Fn(Request) -> ()>) {
    self.handlers
      .borrow_mut()
      .entry(path.to_string())
      .or_insert(Vec::new())
      .push(handler);
  }

  pub fn listen(&self, address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;

    for incoming in listener.incoming() {
      if incoming.is_err() {
        continue
      }

      let mut req = match stream::parse_stream(incoming.unwrap()) {
        Ok(req) => req,
        Err(err) => {
          println!("Error: {err:?}");
          continue;
        }
      };

      println!("Request: {req:?}");
      
      {
        let headers = req.get_headers();
        let accept = headers.accept();
        let sec = headers.get_multi_values("sec-ch-ua");
        println!("Accept: {accept:?}");
        println!("Sec: {sec:?}");
      }


      // let handler = match request::determine_handler(req, self.handlers) {
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