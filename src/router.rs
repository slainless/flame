mod tree;
mod handler;
mod context;
mod params;

use std::{error::Error, io::BufWriter, net::TcpStream, rc::Rc};

use tree::Tree;
pub use context::Context;
pub use handler::{Return, Handler, HandlerFn, HookType};

use crate::{request::Request, response::Response};

pub struct Router {
  tree: Tree
}

impl Router {
  pub fn new() -> Router {
    Router{
      tree: Tree::new()
    }
  }

  pub fn handler(handle: Handle) -> (HandleType, Rc<dyn HandlerFn>) {
    (handle.0, handle.1)
  }

  pub fn register(&mut self, handler: Handler) -> &Self {
    self.tree.register(handler);
    self
  }

  pub fn dispatch(&self, req: Request, stream: TcpStream) -> Result<Response, Box<dyn Error>> {
    let mut response = Response::new(Some(BufWriter::new(stream)));
    let location = req.location();
    let handlers = self.tree.handlers(&location.0, &location.1);
    let (post_handlers, pre_handlers): (Vec<_>, Vec<_>) = handlers
      .into_iter()
      .partition(|h| h.hook_type == HookType::After);

    let error = 'error: {
      let mut ctx: Context = Context{
        req: &req,
        res: &mut response,
        // handler: &handler.clone()
      };

      for handler in pre_handlers {
        let function = handler.function.as_ref();
        match handler.hook_type {
          HookType::Before => {
            match function(&mut ctx) {
              Ok(return_type) => match return_type {
                Return::Next => continue,
                Return::End => {
                  break
                },
              },
              Err(err) => break 'error Some(err)
            }
          },
          HookType::Main => {
            match function(&mut ctx) {
              Ok(return_type) => match return_type {
                  Return::Next => break,
                  Return::End => break,
              },
              Err(err) => break 'error Some(err)
            }
          },
          HookType::After => panic!("Should not dispatch any post handlers here!"),
        }
      }
      
      for handler in post_handlers {
        let function = handler.function.as_ref();
        match handler.hook_type {
          HookType::Before => panic!("Should not dispatch any pre handlers (before) here!"),
          HookType::Main => panic!("Should not dispatch any pre handlers (main) here!"),
          HookType::After => {
            match function(&mut ctx) {
              Ok(return_type) => match return_type {
                Return::Next => continue,
                Return::End => break
              },
              Err(err) => break 'error Some(err)
            }
          }
        }
      }

      None
    };

    match error {
      Some(err) => Err(err),
      None => Ok(response)
    }
  }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HandleType {
  Main,
  Before,
  After,
  Middleware
}

pub struct Handle(HandleType, Rc<dyn HandlerFn>);

impl Handle {
  pub fn main<T: HandlerFn + 'static>(handler: T) -> Handle {
    Handle(HandleType::Main, Rc::new(handler))
  }

  pub fn before<T: HandlerFn + 'static>(handler: T) -> Handle {
    Handle(HandleType::Before, Rc::new(handler))
  }

  pub fn after<T: HandlerFn + 'static>(handler: T) -> Handle {
    Handle(HandleType::After, Rc::new(handler))
  }

  pub fn middleware<T: HandlerFn + 'static>(handler: T) -> Handle {
    Handle(HandleType::Middleware, Rc::new(handler))
  }
}