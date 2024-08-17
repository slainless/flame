use handler::{Handler, HookType, Return};
use tree::Node;
use crate::{request::{Method, Request}, response::Response, router::*, should_debug};
use std::{cell::RefCell, rc::Rc, collections::HashMap};

should_debug!(no);

macro_rules! node {
    ($v:expr) => {
        RefCell::borrow(&$v)
    };
}

macro_rules! parent {
    ($v:expr) => {
        node!($v).parent.upgrade().unwrap()
    };
}

fn __return(status: u32) -> Return {
  let mut res = Response::new();
  res.set_status(status);
  Return::New(res)
}

fn __handler(status: u32, path: &str) -> Handler {
  Handler{
    function: Rc::new(move |_| {
      __return(status)
    }),
    hook_type: HookType::Main,
    method: Method::Get,
    path: path.to_string()
  }
}

fn test_node_exist(tree: &Tree, path: &str) -> Rc<RefCell<Node>> {
  let mut parent = tree.root.clone();
  for (i, fragment) in Tree::split_path(path).into_iter().enumerate() {
    let node = {
      if i == 0 { tree.root.clone() } 
      else {
        parent.as_ref().borrow().next.get(fragment).unwrap().clone()
      }
    };

    let ref_node = node!(node);
    if i == 0 {
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

fn test_handler(h: &Rc<Handler>, status: u32) {
  let result = match h.function.as_ref()(Context { 
    req: Request::new(), 
    res: Response::new(), 
    params: params::new_shared_params(), 
    handler: h.clone()
  }) {
    Return::New(res) => Some(res),
    _ => None
  };
  assert!(result.is_some());
  assert_eq!(result.unwrap().status, status);
}

fn test_handler_exist(node: Rc<RefCell<Node>>, index: usize, priority: u32, status: u32) {
  let node = RefCell::borrow(&node);
  
  assert!(node.handlers.len() > index);
  assert_eq!(node.handlers[index].0, priority);
  test_handler(&node.handlers[index].1, status);
}

mod register_test {
  use super::*;

  #[test]
  fn correct_tree_initialization() {
    let tree = Tree::new();
    let node = node!(tree.root);

    assert_eq!(tree.order, 0);
    assert_eq!(node.fragment, "root");
    assert!(node.parent.upgrade().is_none());
    assert_eq!(node.next.len(), 0);
  }

  #[test]
  fn slashed_prefix_test() {
    let mut tree = Tree::new();
    let path = "/a/b/c/d";

    tree.register(__handler(200, &path));
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

    tree.register(__handler(200, &path));
    assert_eq!(tree.order, 1);
    
    let node = node!(tree.root);
    assert!(node.next.len() == 1);

    let node = test_node_exist(&tree, path);
    test_handler_exist(node.clone(), 0, 0, 200);

    let node = node!(node);
    assert!(node.next.len() == 0);
  }

  #[test]
  fn catch_all_test() {
    let mut tree = Tree::new();
    let path_star = "*";
    let path_normal = "a/b/c/d";

    tree.register(__handler(200, &path_star));
    tree.register(__handler(201, &path_normal));
    tree.register(__handler(202, &path_star));
    tree.register(__handler(203, &path_normal));
    assert_eq!(tree.order, 4);
    
    let ref_node = node!(tree.all);
    assert!(ref_node.handlers.len() == 2);

    test_handler_exist(tree.all.clone(), 0, 0, 200);
    test_handler_exist(tree.all.clone(), 1, 2, 202);
    assert!(ref_node.next.len() == 0);
  }
  
  #[test]
  fn normal_parameter_fragment_test() {
    let mut tree = Tree::new();
    let path_regular = "a/b/c/d";
    let path_parameterized = "a/b/:xyz/d";

    tree.register(__handler(200, &path_regular));
    tree.register(__handler(201, &path_parameterized));
    tree.register(__handler(202, &path_regular));
    tree.register(__handler(203, &path_parameterized));
    assert_eq!(tree.order, 4);

    let node = test_node_exist(&tree, "a/b");
    let node = node!(node);
    assert!(node.next.len() == 2);

    let node = test_node_exist(&tree, &path_regular);
    test_handler_exist(node.clone(), 0, 0, 200);
    test_handler_exist(node.clone(), 1, 2, 202);
    
    let node = test_node_exist(&tree, &path_parameterized);
    test_handler_exist(node.clone(), 0, 1, 201);
    test_handler_exist(node.clone(), 1, 3, 203);

    let node = parent!(node);
    let node = node!(node);
    assert!(node.parameter.as_ref().is_some());
    assert!(node.parameter.as_ref().unwrap().0 == "xyz");
    assert!(node.parameter.as_ref().unwrap().1.is_empty());
  }
  
  #[test]
  fn argumentized_parameter_fragment_test() {
    let mut tree = Tree::new();
    let path_regular = "a/b/c/d";
    let path_parameterized = "a/b/:xyz{this_should_be_regex!}/d";

    tree.register(__handler(200, &path_regular));
    tree.register(__handler(201, &path_parameterized));
    tree.register(__handler(202, &path_regular));
    tree.register(__handler(203, &path_parameterized));
    assert_eq!(tree.order, 4);

    let node = test_node_exist(&tree, "a/b");
    let node = node!(node);
    assert!(node.next.len() == 2);

    let node = test_node_exist(&tree, &path_regular);
    test_handler_exist(node.clone(), 0, 0, 200);
    test_handler_exist(node.clone(), 1, 2, 202);
    
    let node = test_node_exist(&tree, &path_parameterized);
    test_handler_exist(node.clone(), 0, 1, 201);
    test_handler_exist(node.clone(), 1, 3, 203);

    let node = parent!(node);
    let node = node!(node);
    assert!(node.parameter.as_ref().is_some());
    assert!(node.parameter.as_ref().unwrap().0 == "xyz");
    assert!(node.parameter.as_ref().unwrap().1 == "this_should_be_regex!");
  }

  #[test]
  fn wildcard_fragment_test() {
    let mut tree = Tree::new();
    let path_wildcard = "a/b/*/d";
    let path_wildcarddddd = "a/b/***********************/d";

    tree.register(__handler(200, &path_wildcard));
    tree.register(__handler(201, &path_wildcarddddd));
    tree.register(__handler(202, &path_wildcard));
    tree.register(__handler(203, &path_wildcarddddd));
    assert_eq!(tree.order, 4);

    let node = test_node_exist(&tree, "a/b");
    let node = node!(node);
    assert!(node.next.len() == 2);

    let node = test_node_exist(&tree, &path_wildcard);
    test_handler_exist(node.clone(), 0, 0, 200);
    test_handler_exist(node.clone(), 1, 2, 202);
    
    let node = parent!(node);
    let node = node!(node);
    assert!(node.parameter.as_ref().is_none());

    let node = test_node_exist(&tree, &path_wildcarddddd);
    test_handler_exist(node.clone(), 0, 1, 201);
    test_handler_exist(node.clone(), 1, 3, 203);

    let node = parent!(node);
    let node = node!(node);
    assert!(node.parameter.as_ref().is_none());
  }
  
  #[test]
  fn misconfigured_parameter_fragment_test() {
    let mut tree = Tree::new();
    let path_boundary_test = "a/b/:xyz{this_{{{sho}}{}{}uld_be_{{{regex!}/d";
    let path_missing_opening_boundary = "a/b/:xyz this_should_be_regex}}}}/d";
    let path_missing_closing_boundary = "a/b/:xyz{{{{this_should_be_regex/d";
    let path_empty_param = "a/b/:/d";
    let path_multi_start_token = "a/b/:::::wahaha/d";
    let path_multi_slash = "a/b////////////////////////////c///////////////d";

    tree.register(__handler(200, &path_boundary_test));
    tree.register(__handler(201, &path_missing_opening_boundary));
    tree.register(__handler(202, &path_missing_closing_boundary));
    tree.register(__handler(203, &path_empty_param));
    tree.register(__handler(204, &path_multi_start_token));
    tree.register(__handler(205, &path_multi_slash));

    assert_eq!(tree.order, 6);

    let node = test_node_exist(&tree, "a/b");
    let node = node!(node);
    assert!(node.next.len() == 6);

    let node = test_node_exist(&tree, &path_boundary_test);
    test_handler_exist(node.clone(), 0, 0, 200);
    let parent = parent!(node);
    let node = node!(parent);
    assert!(node.parameter.as_ref().is_some());
    assert!(node.parameter.as_ref().unwrap().0 == "xyz");
    assert!(node.parameter.as_ref().unwrap().1 == "this_{{{sho}}{}{}uld_be_{{{regex!");

    let node = test_node_exist(&tree, &path_missing_opening_boundary);
    test_handler_exist(node.clone(), 0, 1, 201);
    let parent = parent!(node);
    let node = node!(parent);
    assert!(node.parameter.as_ref().is_some());
    assert!(node.parameter.as_ref().unwrap().0 == "xyz this_should_be_regex}}}}");
    assert!(node.parameter.as_ref().unwrap().1.is_empty());
    
    let node = test_node_exist(&tree, &path_missing_closing_boundary);
    test_handler_exist(node.clone(), 0, 2, 202);
    let parent = parent!(node);
    let node = node!(parent);
    assert!(node.parameter.as_ref().is_some());
    assert!(node.parameter.as_ref().unwrap().0 == "xyz{{{{this_should_be_regex");
    assert!(node.parameter.as_ref().unwrap().1.is_empty());

    let node = test_node_exist(&tree, &path_empty_param);
    test_handler_exist(node.clone(), 0, 3, 203);
    let parent = parent!(node);
    let node = node!(parent);
    assert!(node.parameter.as_ref().is_some());
    assert!(node.parameter.as_ref().unwrap().0.is_empty());
    assert!(node.parameter.as_ref().unwrap().1.is_empty());
    
    let node = test_node_exist(&tree, &path_multi_start_token);
    test_handler_exist(node.clone(), 0, 4, 204);
    let parent = parent!(node);
    let node = node!(parent);
    assert!(node.parameter.as_ref().is_some());
    assert!(node.parameter.as_ref().unwrap().0 == "::::wahaha");
    assert!(node.parameter.as_ref().unwrap().1.is_empty());
    
    let node = test_node_exist(&tree, &path_multi_slash);
    test_handler_exist(node.clone(), 0, 5, 205);
    let parent_internal = parent!(node);
    let parent = test_node_exist(&tree, "a/b/c");
    assert!(Rc::ptr_eq(&parent, &parent_internal));
    let parent = test_node_exist(&tree, "a/b");
    assert!(RefCell::borrow(&parent).next.get("c").is_some());
  }

  #[test]
  fn deep_tree_integrity_test() {
    let mut tree = Tree::new();
    let path_abcd = "a/b/c/:d";
    let path_abxy = "/a/b/:x/y";
    let path_ab = "a/b";
    let path_xyz = "x/*/z";

    tree.register(__handler(200, &path_abcd));
    tree.register(__handler(300, &path_abxy));
    tree.register(__handler(2, &path_ab));
    tree.register(__handler(200, &path_abxy));
    tree.register(__handler(300, &path_abcd));
    tree.register(__handler(999, &path_abxy));
    tree.register(__handler(199, &path_abcd));
    tree.register(__handler(1, &path_xyz));
    tree.register(__handler(2, &path_xyz));

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

mod tree_routing_test {
  use std::borrow::Borrow;

  use super::*;
  use crate::{dbgln, router::*};

  #[test]
  fn one_path_routing_test() {
    let mut tree = Tree::new();
    let path = "a/b/c/d";

    for i in 0..10 {
      tree.register(__handler(i, &path));
    }

    let handlers = tree.handlers(Method::Get, &path);
    dbgln!("handlers: {:#?}", handlers);
    assert!(handlers.len() == 10);

    for i in 0..10 {
      let (h, param) = handlers.get(i).unwrap();
      test_handler(h, i as u32)
    }
  }

  #[test]
  fn wildcard_routing_test() {
    let mut tree = Tree::new();

    tree.register(__handler(10, "/*/b/c/d"));
    tree.register(__handler(20, "/*/*/c/d"));
    tree.register(__handler(30, "/*/b/*/d"));
    tree.register(__handler(40, "/*/*/*/*"));
    tree.register(__handler(50, "/a/b/c/*"));
    tree.register(__handler(60, "/a/b/c/*"));
    tree.register(__handler(70, "/a/b/c/*"));
    tree.register(__handler(80, "/*/b/c/*"));
    
    let handlers = tree.handlers(Method::Get, "a/b/c/d");
    dbgln!("handlers: {:#?}", handlers);
    assert!(handlers.len() == 8);

    for i in 0..8 {
      let (h, param) = handlers.get(i).unwrap();
      test_handler(h, ((i + 1) * 10) as u32)
    }
  }

  macro_rules! strmap {
    ($( $key:expr => $value:expr ),* $(,)?) => {{
        let mut map: HashMap<_, _> = HashMap::new();
        $(
            map.insert(String::from($key), String::from($value));
        )*
        params::params_from(map)
    }};
}

  #[test]
  fn parameterized_routing_test() {
    let mut tree = Tree::new();

    tree.register(__handler(10, "/:id{regex_goes_BRRRRRR}/b/c/d"));
    tree.register(__handler(20, "/a/:id/c/d"));
    tree.register(__handler(30, "/a/:id{}/:id2{}/d"));
    tree.register(__handler(40, "/:id/:id2{{{{{{{{{{{{{{{{{}}}}}}}}}}}}}}}}}/:id3/:id4"));
    tree.register(__handler(50, "/:id/b/c/:id2"));
    tree.register(__handler(60, "/a/b/c/d"));
    tree.register(__handler(70, "/a/b/c/d"));
    tree.register(__handler(80, "/a/b/c/:id{{{{{{{{{{{{{{{{{{{XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX]}"));
    
    let handlers = tree.handlers(Method::Get, "a/b/c/d");
    dbgln!("handlers: {:#?}", handlers);
    assert!(handlers.len() == 8);

    for i in 0..8 {
      let (h, param) = handlers.get(i).unwrap();
      test_handler(h, ((i + 1) * 10) as u32);

      match i {
        0 => assert!(param.clone().as_ref() == &strmap!("id" => "a")),
        1 => assert!(param.clone().as_ref() == &strmap!("id" => "b")),
        2 => assert!(param.clone().as_ref() == &strmap!("id" => "b", "id2" => "c")),
        3 => assert!(param.clone().as_ref() == &strmap!(
          "id" => "a", 
          "id2" => "b", 
          "id3" => "c",
          "id4" => "d"
        )),
        4 => assert!(param.clone().as_ref() == &strmap!("id" => "a", "id2" => "d")),
        5 => assert!(param.clone().as_ref() == &strmap!()),
        6 => assert!(param.clone().as_ref() == &strmap!()),
        7 => assert!(param.clone().as_ref() == &strmap!("id" => "d")),
        _ => assert!(false, "Should be unreachable"),
      }
    }
  }

  #[test]
  fn routing_integrity_test() {
    let mut tree = Tree::new();

    tree.register(__handler(0, "/a/b/c/d"));
    tree.register(__handler(10, "/a/b/:id/d"));
    tree.register(__handler(20, "/a/*/c/d"));
    tree.register(__handler(30, "/a/*/c/d"));
    tree.register(__handler(0, "/*/b/*should_not_match/d"));
    tree.register(__handler(0, "/*/b/*should_not_match/d"));
    tree.register(__handler(0, "/*/b/*should_not_match/d"));
    tree.register(__handler(40, "/*/b/c/d"));
    tree.register(__handler(50, "/*/b/c/d"));
    tree.register(__handler(60, "/*/b/:goes_brrrr}}}}}}}}/d"));
    tree.register(__handler(70, "/a/::huzzah!!{{{{{{{/:hahah{rrrr}/d"));
    tree.register(__handler(80, "/*/*/*/*"));
    tree.register(__handler(90, "/*/b/:hahah{rrrr}/d"));
    tree.register(__handler(100, "/:multiple/:param/:goes{rrrr}/:brrrr"));

    let handlers = tree.handlers(Method::Get, "/a/b/c/d");

    assert!(handlers.len() == 11);

    for i in 0..11 {
      let (h, param) = handlers.get(i).unwrap();
      test_handler(h, (i * 10) as u32);

      match i {
        0 => assert!(param.clone().as_ref() == &strmap!()),
        1 => assert!(param.clone().as_ref() == &strmap!("id" => "c")),
        2 => assert!(param.clone().as_ref() == &strmap!()),
        3 => assert!(param.clone().as_ref() == &strmap!()),
        4 => assert!(param.clone().as_ref() == &strmap!()),
        5 => assert!(param.clone().as_ref() == &strmap!()),
        6 => assert!(param.clone().as_ref() == &strmap!("goes_brrrr}}}}}}}}" => "c")),
        7 => assert!(param.clone().as_ref() == &strmap!(
          ":huzzah!!{{{{{{{" => "b",
          "hahah" => "c"
        )),
        8 => assert!(param.clone().as_ref() == &strmap!()),
        9 => assert!(param.clone().as_ref() == &strmap!("hahah" => "c")),
        10 => assert!(param.clone().as_ref() == &strmap!(
          "multiple" => "a",
          "param" => "b",
          "goes" => "c",
          "brrrr" => "d",
        )),
        _ => assert!(false, "Should be unreachable"),
      }
    }
  }

  #[test]
  fn hook_priority_test() {
    let mut tree = Tree::new();

    
  }
}