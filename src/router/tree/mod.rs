use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};
use crate::{dbgln, request::Method, router::params, should_debug};
use super::{handler::{Handler, SharedHandler}, params::{params_from, SharedParams}};

mod node;
use node::*;

should_debug!(0);

#[cfg(test)]
mod tree_test;

#[derive(Debug)]
pub struct Tree {
  order: u32,
  root: MutSharedNode,
  all: MutSharedNode
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      order: 0,
      root: Node::new_mut_shared("root"), 
      all: Node::new_mut_shared("*"),
    }
  }

  pub fn register(&mut self, handler: Handler) -> &Self {
    if &handler.path == "*" {
      let all = self.all.clone();
      let mut all = RefCell::borrow_mut(&all);
      all.handlers.push((self.order, Rc::new(handler)));
      self.order += 1;
      return self;
    }

    let last_node = 'node: {
      let mut parent = self.root.clone();

      let fragments = Tree::split_path(&handler.path);
      if fragments.len() == 1 {
        break 'node parent
      }
  
      for (i, fragment) in fragments.iter().enumerate() {
        if i == 0 {
          continue
        }

        let node = {
          let node = RefCell::borrow_mut(&parent)
            .next
            .entry(fragment.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(fragment))))
            .clone();

          RefCell::borrow_mut(&node).parent = Rc::downgrade(&parent);
          node
        };

        parent = node;
      }

      break 'node parent
    };
    
    RefCell::borrow_mut(&last_node)
      .handlers.push((self.order, Rc::new(handler)));

    self.order += 1;
    self
  }
  
  pub fn handlers(&self, method: &Method, path: &str) -> Vec<(SharedHandler, SharedParams)> {
    let fragments = Tree::split_path(path);

    let all = RefCell::borrow(&self.all);
    let handlers = RefCell::new(Vec::new());

    {
      let mut h = RefCell::borrow_mut(&handlers);
      let empty_param = params::new_shared_params();
  
      h.extend_from_slice(
        &all.handlers
          .iter()
          .map(|h| ((h.0, h.1.clone()), empty_param.clone()))
          .collect::<Vec<_>>()
      );
    }

    if fragments.len() == 1 {
      let node = RefCell::borrow(&self.root);
      let mut h = RefCell::borrow_mut(&handlers);
      let empty_param = params::new_shared_params();

      h.extend_from_slice(
        &node.handlers
          .iter()
          .map(|h| ((h.0, h.1.clone()), empty_param.clone()))
          .collect::<Vec<_>>()
      );
    } else {
      let root = RefCell::borrow(&self.root);
      Tree::traverse(
        &fragments,
        &root.next,
        1, // we start at 1 to skip root...
        HashMap::new(),
        &|node, parameters, fragments, i| {
          dbgln!("on_match hook called with index: {}", i);
          if fragments.len() == i {
            let node = RefCell::borrow(&node);
            dbgln!("Node at last fragment ({}): {:#?}", fragments[i - 1], node);
            let mut h = RefCell::borrow_mut(&handlers);
            let param = Rc::new(Tree::build_parameters(parameters));
             
            let h2 = node.handlers
                .iter()
                .map(|h| ((h.0, h.1.clone()), param.clone()))
                .collect::<Vec<_>>();

            h.extend_from_slice(&h2);
          }
        }
      )
    }

    dbgln!("Handlers: {:#?}", handlers);

    let mut h = handlers.take();
    h.sort_by(|a, b| {
      Tree::compare_handler(&a.0, &b.0)
    });
    h
      .iter()
      .map(|h| (h.0.1.clone(), h.1.clone()))
      .filter(|h| {
        h.0.method == *method || *method == Method::All
      })
      .collect()
  }

  fn build_parameters(param: &HashMap<&str, &str>) -> params::Params {
    params_from(param
      .iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
      .collect::<HashMap<String, String>>()
    )
  }

  // will always return "" as the first fragment
  // whatever the input is
  fn split_path(path: &str) -> Vec<&str> {
    if path.is_empty() || path == "/" {
      vec![""]
    } else {
      let path: Vec<_> = path
        .split('/')
        .filter(|x| !x.is_empty())
        .collect();
      let mut vec = vec![""];
      vec.extend_from_slice(&path);
      vec
    }
  }

  fn compare_handler(a: &PrioritizedHandler, b: &PrioritizedHandler) -> Ordering {
    if &a.1.hook_type == &b.1.hook_type {
      a.0.cmp(&b.0)
    }
    else {
      a.1.hook_type.cmp(&b.1.hook_type)
    }
  }
  
  fn rebuild_path_to_root(node: Rc<RefCell<Node>>) -> String {
    let mut node = node;
    let mut path = String::new();
    loop {
      let cloned_node = node.clone();
      let ref_node = RefCell::borrow(&cloned_node);
      let parent = ref_node.parent.upgrade();
      if parent.is_none() {
        break
      }

      let parent = parent.unwrap();
      path.insert_str(0, &ref_node.fragment);
      path.insert_str(0, "/");
      node = parent.clone();
    }

    path
  }

  fn traverse(
    fragments: &[&str], 
    next: &HashMap<String, Rc<RefCell<Node>>>,
    i: usize,
    parameters: HashMap<&str, &str>,
    on_match: &impl Fn(&Rc<RefCell<Node>>, &HashMap<&str, &str>, &[&str], usize),
  ) {
    dbgln!("Start traversing with index: {}", i);
    let cursor = {
      if let Some(i) = fragments.get(i) {
        *i
      } else {
        return
      }
    };

    dbgln!("Traversing <{}>, of {:?}", cursor, fragments.join("/"));
    for (_, node) in next {
      let ref_node = RefCell::borrow(&node);
      // if current node is a normal fragment and cursor is not matched
      // then return early
      dbgln!("Testing cursor <{}> against fragment <{}> for path <{}>", 
        cursor, ref_node.fragment, Tree::rebuild_path_to_root(node.clone()));
      if ref_node.parameter.is_none() && ref_node.fragment != "*" && cursor != ref_node.fragment {
        dbgln!("Cursor <{}> does not match fragment <{}> for path <{}>", 
          cursor, ref_node.fragment, Tree::rebuild_path_to_root(node.clone()));
        continue
      }

      let derived_next: &HashMap<String, Rc<RefCell<Node>>> = &ref_node.next;
      let mut derived_parameters: HashMap<&str, &str> = HashMap::new();
      derived_parameters.clone_from(&parameters);

      let derived_i = {
        // handler for special fragment (e.g. ":id", ":id{regex}")
        if let Some(parameter) = &ref_node.parameter {
          dbgln!("Cursor <{}> match parameterized fragment <{}> for path <{}>", 
            cursor, ref_node.fragment, Tree::rebuild_path_to_root(node.clone()));
          derived_parameters.insert(parameter.0.as_str(), cursor);
          i + 1
        // handler for normal url fragment
        } else {
          dbgln!("Cursor <{}> match normal fragment <{}> for path <{}>", 
            cursor, ref_node.fragment, Tree::rebuild_path_to_root(node.clone()));
          i + 1
        }
      };

      dbgln!("Calling on_match hook since cursor matched");
      on_match(&node, &derived_parameters, fragments, derived_i);

      Tree::traverse(
        fragments, 
        derived_next, 
        derived_i,
        derived_parameters,
        on_match
      );
    }   
  }
}
