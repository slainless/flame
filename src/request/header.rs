use std::collections::HashMap;

mod parser;

macro_rules! values {
  ($source:expr, $key:expr) => {
      {
          $source.get($key).and_then(|v| Some(parser::multi_values_field(v)))
      }
  };
}

macro_rules! value {
($source:expr, $key:expr) => {
    {
        $source.get($key).and_then(|v| parser::single_value_field(v))
    }
};
}

#[derive(Debug)]
pub struct Headers {
  headers: HashMap<String, Vec<String>>
}

impl Headers {
  pub fn new() -> Headers {
    Headers{
      headers: HashMap::new()
    }
  }

  pub fn set(&mut self, key: String, value: String) -> &Self {
    let value = vec![value];
    self.headers.insert(key, value);
    self
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.headers.get(key).and_then(|v| Some(&v[0]))
  }

  pub fn get_all(&self, key: &str) -> Option<&Vec<String>> {
    self.headers.get(key)
  }

  pub fn append(&mut self, key: String, value: String) -> &Self {
    self.headers
      .entry(key)
      .or_default()
      .push(value);
    self
  }

  pub fn accept(&self) -> Option<Vec<Value>> {
    values!(self, "accept")
  }

  pub fn accept_encoding(&self) -> Option<Vec<Value>> {
    values!(self, "accept-encoding")
  }

  pub fn content_type(&self) -> Option<Value> {
    value!(self, "content-type")
  }

  pub fn content_length(&self) -> Option<usize> {
    value!(self, "content-length")
      .and_then(|v| v.0.parse().ok())
  }

  pub fn get_value(&self, key: &str) -> Option<Value> {
    self
      .get(key)
      .and_then(|v| 
        parser::single_value_field(v)
      )
  }

  pub fn get_multi_values(&self, key: &str) -> Option<Vec<Value>> {
    values!(self, key)
  }

  pub fn get_multi_values_all(&self, key: &str) -> Option<Vec<Vec<Value>>> {
    self
      .get_all(key)
      .and_then(|v| v.iter()
        .map(|v| Some(parser::multi_values_field(v)))
        .collect()
      )
  }
}

#[derive(Debug)]
pub struct Value<'a>(&'a str, HashMap<&'a str, &'a str>);
