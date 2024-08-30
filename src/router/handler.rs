use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::request::Method;
use crate::response::Response;

use super::Context;

pub enum Return {
  Next,
  End
}

pub trait HandlerFn: Fn(&mut Context) -> Result<Return, Box<dyn Error>> {}
impl <F> HandlerFn for F where F: Fn(&mut Context) -> Result<Return, Box<dyn Error>> {}
impl std::fmt::Debug for dyn HandlerFn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "HandlerFn")
  }
}

#[derive(Debug, Clone)]
pub struct Handler {
  pub method: Method,
  pub path: String,
  pub function: Rc<dyn HandlerFn>,
  pub hook_type: HookType
}

impl Handler {
}

pub type SharedHandler = Rc<Handler>;
pub type MutSharedHandler = Rc<RefCell<Handler>>;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum HookType {
  Before,
  Main,
  After
}