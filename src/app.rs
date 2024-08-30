use std::{error::Error, net::TcpListener};

use crate::{request::Method, router::{HandleType, Handler, HookType, Router}, stream, Handle};

pub struct App {
  router: Router
}

impl App {
  pub fn new() -> App {
    App{
      router: Router::new()
    }
  }

  pub fn all(&mut self, path: &str, handle: Handle) -> &Self {
    self.register_handle(Method::All, path, handle)
  }

  pub fn get(&mut self, path: &str, handle: Handle) -> &Self {
    self.register_handle(Method::Get, path, handle)
  }

  pub fn post(&mut self, path: &str, handle: Handle) -> &Self {
    self.register_handle(Method::Post, path, handle)
  }

  fn register_handle(&mut self, method: Method, path: &str, handle: Handle) -> &Self {
    let (hook_type, function) = Router::handler(handle);
    let mut handler = Handler{
      method, 
      path: path.to_string(), 
      function, 
      hook_type: HookType::Main 
    };

    match hook_type {
      HandleType::Main => {
        handler.hook_type = HookType::Main;
      },
      HandleType::After => {
        handler.hook_type = HookType::After;
      },
      HandleType::Before => {
        handler.hook_type = HookType::Before;
      },
      HandleType::Middleware => {
        let mut another_handler = handler.clone();
        handler.hook_type = HookType::Before;
        another_handler.hook_type = HookType::After;

        self.router.register(handler);
        self.router.register(another_handler);
        return self;
      }
    }

    self.router.register(handler);
    self
  }

  pub fn listen(&self, address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;

    for incoming in listener.incoming() {
      let req_stream = match incoming {
        Ok(stream) => stream,
        Err(_) => {
          continue;
        }
      };
      let res_stream = req_stream.try_clone()?;

      let req = match stream::parse_stream(req_stream) {
        Ok(req) => req,
        Err(err) => {
          println!("Error parsing stream: {err:?}");
          continue;
        }
      };

      self.router.dispatch(req, res_stream);
    }

    return Ok(())
  }
}