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
  parent: Weak<RefCell<Node>>
}

impl Node {
  pub fn new(fragment: &str) -> Node {
    Node {
      fragment: fragment.to_string(),
      handlers: Vec::new(),
      next: HashMap::new(),
      parent: Weak::new()
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
            node.parent = Rc::downgrade(&parent);
          }

          node.clone()
        };

        parent = node;
      }

      break 'node parent
    };
    
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

  macro_rules! parent {
      ($v:expr) => {
          node!($v).parent.upgrade().unwrap()
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
    assert!(node.parent.upgrade().is_none());
    assert_eq!(node.next.len(), 0);
  }

  fn test_node_exist(tree: &Tree, path: &str) -> Rc<RefCell<Node>> {
    let mut parent = tree.root.clone();
    for (i, fragment) in path.split("/").into_iter().enumerate() {
      let node = {
        if i == 0 && fragment == "" { tree.root.clone() } 
        else {
          parent.as_ref().borrow().next.get(fragment).unwrap().clone()
        }
      };

      let ref_node = node!(node);
      if i == 0 && fragment == "" {
        assert!(ref_node.parent.upgrade().is_none());
        assert!(ref_node.next.len() >= 1);
        continue
      }

      assert!(ref_node.parent.upgrade().is_some());
      assert!(Rc::ptr_eq(&parent, &parent!(node)));
      assert!(ref_node.fragment == fragment);
    
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
  fn slashed_prefix_test() {
    let mut tree = Tree::new();
    let path = "/a/b/c/d";

    tree.register(handl!(200, &path));
    assert_eq!(tree.order, 1);

    let node = node!(tree.root);
    assert!(node.next.len() == 1);
    
    let node = test_node_exist(&tree, path);
    test_handler_exist(node.clone(), 0, 0, 200);
    
    let node = node!(node);
    assert!(node.next.len() == 0);
  }

  #[test]
  fn unslashed_prefix_test() {
    let mut tree = Tree::new();
    let path = "a/b/c/d";

    tree.register(handl!(200, &path));
    assert_eq!(tree.order, 1);
    
    let node = node!(tree.root);
    assert!(node.next.len() == 1);

    let node = test_node_exist(&tree, path);
    test_handler_exist(node.clone(), 0, 0, 200);

    let node = node!(node);
    assert!(node.next.len() == 0);
  }

  #[test]
  fn deep_tree_integrity_test() {
    let mut tree = Tree::new();
    let path_abcd = "a/b/c/d";
    let path_abxy = "/a/b/x/y";
    let path_ab = "a/b";
    let path_xyz = "x/y/z";

    tree.register(handl!(200, &path_abcd));
    tree.register(handl!(300, &path_abxy));
    tree.register(handl!(2, &path_ab));
    tree.register(handl!(200, &path_abxy));
    tree.register(handl!(300, &path_abcd));
    tree.register(handl!(999, &path_abxy));
    tree.register(handl!(199, &path_abcd));
    tree.register(handl!(1, &path_xyz));
    tree.register(handl!(2, &path_xyz));

    let ref_node_root = node!(tree.root);
    assert!(ref_node_root.next.len() == 2);

    let node_ab = test_node_exist(&tree, path_ab);
    test_handler_exist(node_ab.clone(), 0, 2, 2);

    let ref_node_ab = node!(node_ab);
    assert!(ref_node_ab.next.len() == 2);
    assert!(ref_node_ab.handlers.len() == 1);

    let node_abcd = test_node_exist(&tree, path_abcd);
    test_handler_exist(node_abcd.clone(), 0, 0, 200);
    test_handler_exist(node_abcd.clone(), 1, 4, 300);
    test_handler_exist(node_abcd.clone(), 2, 6, 199);

    let ref_node_abcd = node!(node_abcd);
    assert!(ref_node_abcd.next.len() == 0);
    assert!(ref_node_abcd.handlers.len() == 3);
    assert!(Rc::ptr_eq(&node_ab, &parent!(parent!(node_abcd))));

    let node_abxy = test_node_exist(&tree, path_abxy);
    test_handler_exist(node_abxy.clone(), 0, 1, 300);
    test_handler_exist(node_abxy.clone(), 1, 3, 200);
    test_handler_exist(node_abxy.clone(), 2, 5, 999);

    let ref_node_abxy = node!(node_abxy);
    assert!(ref_node_abxy.next.len() == 0);
    assert!(ref_node_abxy.handlers.len() == 3);
    assert!(Rc::ptr_eq(&node_ab, &node!(ref_node_abxy.parent.upgrade().unwrap()).parent.upgrade().unwrap()));

    let node_xyz = test_node_exist(&tree, path_xyz);
    test_handler_exist(node_xyz.clone(), 0, 7, 1);
    test_handler_exist(node_xyz.clone(), 1, 8, 2);
    
    let ref_node_xyz = node!(node_xyz);
    assert!(ref_node_xyz.next.len() == 0);
    assert!(ref_node_xyz.handlers.len() == 2);
    assert!(Rc::ptr_eq(&tree.root, &parent!(parent!(parent!(node_xyz)))));
  }
}