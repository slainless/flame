use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::{Rc, Weak}};

use super::Handler;

#[derive(Debug)]
pub struct Tree {
  order: u32,
  root: Rc<RefCell<Node>>
}

#[derive(Debug)]
struct Node {
  fragment: String,
  handlers: Vec<(u32, Handler)>,
  next: HashMap<String, Rc<RefCell<Node>>>,
  parent: Option<Weak<RefCell<Node>>>
}

impl Node {
  pub fn new(fragment: &str) -> Node {
    Node {
      fragment: fragment.to_string(),
      handlers: Vec::new(),
      next: HashMap::new(),
      parent: None
    }
  }
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      order: 0,
      root: Rc::new(RefCell::new(Node::new("root"))), 
    }
  }

  pub fn register(&mut self, handler: Handler) -> &Self {
    let last_node = 'node: {
      let mut parent = self.root.clone();

      let fragments: Vec<&str> = handler.path.split("/").collect();
      if fragments.len() == 1 && fragments[0] == "" {
        break 'node parent
      }
  
      for (i, fragment) in fragments.iter().enumerate() {
        if i == 0 && *fragment == "" {
          continue
        }

        let node = {
          let mut __parent = parent
            .as_ref()
            .borrow_mut();
          
          let node = __parent
            .next
            .entry(fragment.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(fragment))));

          {
            let mut node = node.as_ref().borrow_mut();
            node.parent = Some(Rc::downgrade(&parent));
          }

          node.clone()
        };

        parent = node;
      }

      break 'node parent
    };

    println!("{:?}", self.root);
    
    last_node.as_ref().borrow_mut().handlers.push((self.order, handler));
    self.order += 1;
    self
  }
}

#[cfg(test)]
mod tree_test {
  use super::*;
  use crate::router::*;

  macro_rules! node {
      ($v:expr) => {
          $v.as_ref().borrow()
      };
  }

  macro_rules! ret {
      ($v:expr) => {
          {
            let mut res = Response::new();
            res.set_status($v);
            Return::New(res)
          }
      };
  }
  
  macro_rules! handl {
    ($v:expr, $path:expr) => {
      {
        Handler{
          handler: Box::new(|_, _| ret!($v)),
          hook: Hook::Main,
          method: Method::Get,
          path: $path.to_string()
        }
      }
    }
  }

  #[test]
  fn correct_tree_initialization() {
    let tree = Tree::new();
    let node = node!(tree.root);

    assert_eq!(tree.order, 0);
    assert_eq!(node.fragment, "root");
    assert!(node.parent.is_none());
    assert_eq!(node.next.len(), 0);
  }

  fn test_node_exist(tree: &Tree, path: &str) -> Rc<RefCell<Node>> {
    let mut parent = tree.root.clone();
    for (i, fragment) in path.split("/").into_iter().enumerate() {
      let node = {
        println!("frag: {}, index: {}", fragment, i);
        if i == 0 && fragment == "" { tree.root.clone() } 
        else {
          parent.as_ref().borrow().next.get(fragment).unwrap().clone()
        }
      };

      let ref_node = node!(node);
      if i == 0 && fragment == "" {
        assert!(ref_node.parent.is_none());
        assert_eq!(ref_node.next.len(), 1);
        continue
      }

      assert!(ref_node.parent.is_some());
      let parent_interior = ref_node.parent.clone().unwrap();
      assert!(Rc::ptr_eq(&parent, &parent_interior.upgrade().unwrap()));
      assert!(ref_node.fragment == fragment);

      if i == path.split("/").count() - 1 {
        assert!(ref_node.next.len() == 0);
      }
    
      parent = node.clone();
    }

    parent
  }

  fn test_handler_exist(node: Rc<RefCell<Node>>, index: usize, priority: u32, status: u32) {
    let node = node.as_ref().borrow();
    
    assert!(node.handlers.len() > index);
    assert_eq!(node.handlers[index].0, priority);
    let handler = &node.handlers[index].1.handler;
    let result = match handler(Request::new(), &node.handlers[index].1) {
      Return::New(res) => Some(res),
      _ => None
    };
    assert!(result.is_some());
    assert_eq!(result.unwrap().status, status);
  }

  #[test]
  fn simple_tree_insertion_slashed() {
    let mut tree = Tree::new();
    let path = "/a/b/c/d";

    tree.register(handl!(200, &path));
    assert_eq!(tree.order, 1);
    
    let node = test_node_exist(&tree, path);
    test_handler_exist(node, 0, 0, 200);
  }

  #[test]
  fn simple_tree_insertion_unslashed() {
    let mut tree = Tree::new();
    let path = "a/b/c/d";

    tree.register(handl!(200, &path));
    assert_eq!(tree.order, 1);
    
    let node = test_node_exist(&tree, path);
    test_handler_exist(node, 0, 0, 200);
  }
}