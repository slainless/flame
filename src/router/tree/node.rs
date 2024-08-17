use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};

use crate::router::handler::SharedHandler;

pub type PrioritizedHandler = (u32, SharedHandler);
pub type FragmentParameter = Option<(String, String)>;

#[derive(Debug)]
pub struct Node {
  pub fragment: String,
  pub parameter: FragmentParameter,
  pub handlers: Vec<PrioritizedHandler>,
  pub next: HashMap<String, MutSharedNode>,
  pub parent: WeakSharedNode
}

impl Node {
  pub fn new(fragment: &str) -> Node {
    Node {
      fragment: fragment.to_string(),
      parameter: Node::parse_parameter(fragment),
      handlers: Vec::new(),
      next: HashMap::new(),
      parent: Weak::new()
    }
  }

  pub fn new_mut_shared(fragment: &str) -> MutSharedNode {
    Rc::new(RefCell::new(Node::new(fragment)))
  }

  fn parse_parameter(fragment: &str) -> FragmentParameter {
    if !fragment.starts_with(":") {
      return None
    }

    if !fragment.ends_with("}") {
      return Some((fragment[1..].to_string(), String::new()))
    }

    if let Some((key,value)) = fragment.split_once("{") {
      return Some((key[1..].to_string(), value[0..value.len() - 1].to_string()))
    } else {
      return Some((fragment[1..].to_string(), String::new()))
    }
  }
}

pub type MutSharedNode = Rc<RefCell<Node>>;
pub type WeakSharedNode = Weak<RefCell<Node>>;