use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Params(HashMap<String, String>);

impl Params {
  pub fn get(&self, key: &str) -> Option<&str> {
    self.0.get(key).map(|v| v.as_str())
  }

  fn insert(&mut self, key: String, value: String) {
    self.0.insert(key, value);
  }
}

pub fn params_from(map: HashMap<String, String>) -> Params {
  Params(map)
}

pub fn set_params(param: &mut Params, key: String, value: String) {
  param.insert(key, value);
}

pub fn new_params() -> Params {
  Params(HashMap::new())
}

pub fn new_shared_params() -> SharedParams {
  Rc::new(Params(HashMap::new()))
}

impl Clone for Params {
  fn clone(&self) -> Self {
    Params(self.0.clone())
  }
}

impl PartialEq for Params {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl Eq for Params {}

pub type SharedParams = Rc<Params>;