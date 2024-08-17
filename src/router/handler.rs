use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::request::Method;
use crate::response::Response;

use super::Context;

pub enum Return {
  Next,
  Merge(Response),
  New(Response)
}

pub trait HandlerFn: Fn(Context) -> Return {}
impl <F> HandlerFn for F where F: Fn(Context) -> Return {}
impl std::fmt::Debug for dyn HandlerFn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "HandlerFn")
  }
}

#[derive(Debug)]
pub struct Handler {
  pub method: Method,
  pub path: String,
  pub function: Box<dyn HandlerFn>,
  pub hook_type: HookType
}

impl Handler {
}

pub type SharedHandler = Rc<Handler>;
pub type MutSharedHandler = Rc<RefCell<Handler>>;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HookType {
  Before,
  Main,
  After
}