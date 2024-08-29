mod tree;
mod handler;
mod context;
mod params;

use std::{borrow::Borrow, rc::Rc};

use tree::Tree;
pub use context::Context;
pub use handler::{Return, Handler, HandlerFn, HookType};

use crate::{request::Request, response::{self, Response}};

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

  pub fn dispatch(&self, req: Request) -> Response {
    let mut final_response = Response::new();
    let location = req.location();
    let handlers = self.tree.handlers(&location.0, &location.1);
    let (post_handlers, pre_handlers): (Vec<_>, Vec<_>) = handlers
      .into_iter()
      .partition(|h| h.hook_type == HookType::After);

    for handler in pre_handlers {
      let ctx = Context{
        req: &req,
        res: &mut final_response,
        handler: &handler.clone()
      };

      let function = handler.function.as_ref();
      match handler.hook_type {
        HookType::Before => {
          match function(ctx) {
            Return::Next => continue,
            Return::Merge(response) => {
              response::merge_move(response, &mut final_response);
              break
            },
            Return::New(response) => {
              final_response = response;
              break
            }
          }
        },
        HookType::Main => {
          match function(ctx) {
            Return::Next => (),
            Return::Merge(response) => response::merge_move(response, &mut final_response),
            Return::New(response) => final_response = response,
          }

          break
        },
        HookType::After => panic!("Should not dispatch any post handlers here!"),
      }
    }
    
    for handler in post_handlers {
      let ctx = Context{
        req: &req,
        res: &mut final_response,
        handler: &handler.clone()
      };

      let function = handler.function.as_ref();
      match handler.hook_type {
        HookType::Before => panic!("Should not dispatch any pre handlers (before) here!"),
        HookType::Main => panic!("Should not dispatch any pre handlers (main) here!"),
        HookType::After => {
          match function(ctx) {
            Return::Next => continue,
            Return::Merge(response) => {
              response::merge_move(response, &mut final_response);
              break
            },
            Return::New(response) => {
              final_response = response;
              break
            }
          }
        }
      }
    }

    final_response
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