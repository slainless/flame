mod tree;
mod handler;
mod context;
mod params;

use std::rc::Rc;

use tree::Tree;
pub use context::Context;
pub use params::{Params, SharedParams};
pub use handler::{Return, Handler, HandlerFn, HookType};

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