use crate::request::{Method, Request};
use crate::response::Response;

pub enum Return {
  Next,
  Merge(Response),
  New(Response)
}

#[derive(Debug)]
pub enum Hook {
  Before,
  After,
  Main
}

pub trait HandlerFn: Fn(Request, &Handler) -> Return {}
impl <F> HandlerFn for F where F: Fn(Request, &Handler) -> Return {}
impl std::fmt::Debug for dyn HandlerFn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "HandlerFn")
  }
}

#[derive(Debug)]
pub struct Handler {
  pub method: Method,
  pub path: String,
  pub handler: Box<dyn HandlerFn>,
  pub hook: Hook
}

mod tree;
use tree::Tree;

pub struct Router {
  tree: Tree
}

impl Router {
  pub fn new() -> Router {
    Router{
      tree: Tree::new()
    }
  }

  pub fn register(&mut self, handler: Handler) -> &Self {
    self.tree.register(handler);
    self
  }

  // pub fn handle(&self, req: Request) -> Vec<&Handler> {
    
  // }
}