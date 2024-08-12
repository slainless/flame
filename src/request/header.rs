use std::collections::HashMap;
use crate::multi_value;

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
    multi_value!(self, "accept")
  }

  pub fn accept_encoding(&self) -> Option<Vec<Value>> {
    multi_value!(self, "accept-encoding")
  }

  // pub fn content_type(&self) -> Option<Value> {
  //   self
      
  // }

  pub fn get_multi_values(&self, key: &str) -> Option<Vec<Value>> {
    multi_value!(self, key)
  }

  pub fn get_multi_values_all(&self, key: &str) -> Option<Vec<Vec<Value>>> {
    self
      .get_all(key)
      .and_then(|v| v.iter()
        .map(|v| Some(parse_multi_value(v)))
        .collect()
      )
  }
}

#[derive(Debug)]
pub struct Value<'a>(&'a str, HashMap<&'a str, &'a str>);

fn field_split(value: &str, delimiter: char) -> Vec<&str> {
  let mut values: Vec<&str> = Vec::new();
  
  let mut should_skip = false;
  
  let mut start = 0;
  for (index, char) in value.char_indices() {
    if char == '\"' {
      if index == 0 || value.get(index-1..index-1) != Some("\\") {
        should_skip = !should_skip;
        continue;
      }
    }

    if char == delimiter && !should_skip {
      values.push(&value[start..index]);
      start = index + 1;
    }
  }

  if start != value.len() {
    values.push(&value[start..]);
  }

  values
}

// fn field_split_n(value: &str, delimiter: char, n: usize) -> Vec<&str> {
//   let mut values: Vec<&str> = Vec::new();
  
//   let mut should_skip = false;
  
//   let mut start = 0;
//   for (index, char) in value.char_indices() {
//     if char == '\"' {
//       if index == 0 || value.get(index-1..index-1) != Some("\\") {
//         should_skip = !should_skip;
//         continue;
//       }
//     }

//     if char == delimiter && !should_skip {
//       values.push(&value[start..index]);
//       if values.len() == n {
//         break;
//       }
//       start = index + 1;
//     }
//   }

//   if start != value.len() {
//     values.push(&value[start..]);
//   }

//   values
// }

fn parse_multi_value(value: &str) -> Vec<Value> {
  field_split(value, ',')
    .iter()
    .map(|v| {
      let splitted = field_split(v, ';');
      if splitted.len() == 1 {
        Value(splitted[0].trim(), HashMap::new())
      } else {
        let mut map: HashMap<&str, &str> = HashMap::new();
        for entry in splitted[1..].iter() {
          if let Some((key, value)) = entry.split_once('=') {
            map.insert(key.trim(), value.trim());
          } else {
            map.insert(v.trim(), "");
          }
        }
        Value(splitted[0].trim(), map)
      }
    })
    .collect()
}

#[macro_export]
macro_rules! multi_value {
    ($source:expr, $key:expr) => {
        {
            $source.get($key).and_then(|v| Some(parse_multi_value(v)))
        }
    };
}