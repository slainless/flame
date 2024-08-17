use std::collections::HashMap;

use super::Value;

pub fn value(value: &str) -> Value {
  let splitted = field_string_split_all(value, ';');
  // got only the field value
  if splitted.len() == 1 {
    Value(trim_field_value(splitted[0]), HashMap::new())
  } 
  // got field value and parameters
  else {
    let mut map: HashMap<&str, &str> = HashMap::new();
    for entry in splitted[1..].iter() {
      // split parameter key and value
      let parameter = field_string_split_max_n(entry, '=', 2);
      if parameter.len() == 1 {
        map.insert(trim_field_value(parameter[0]), "");
      } else {
        map.insert(trim_field_value(parameter[0]), trim_field_value(parameter[1]));
      }
    }
    Value(trim_field_value(splitted[0]), map)
  }
}

pub fn multi_values_field(field: &str) -> Vec<Value> {
  field_string_split_all(field, ',')
    .iter()
    .map(|v| value(v))
    .collect()
}

pub fn single_value_field(field: &str) -> Option<Value> {
  field_string_split_max_n(field, ',', 1)
    .get(0)
    .and_then(|v| Some(value(v)))
}

fn field_string_split_all(value: &str, delimiter: char) -> Vec<&str> {
  field_string_split_max_n(value, delimiter, usize::MAX)
}

fn field_string_split_max_n(value: &str, delimiter: char, n: usize) -> Vec<&str> {
  if n == 0 {
    panic!("n must be greater than 0");
  }

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
      if values.len() >= n {
        return values;
      }
      start = index + 1;
    }
  }

  if start != value.len() {
    values.push(&value[start..]);
  }

  values
}

fn trim_field_value(value: &str) -> &str {
  let str = value.trim();
  if str.starts_with('\"') && str.ends_with('\"') {
    &str[1..str.len()-1]
  } else {
    &str
  }
}