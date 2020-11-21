use std::collections::HashMap;

type Function = fn(Vec<String>) -> String;
pub struct Scope {
  records: HashMap<String, Function>
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      records: HashMap::new()
    }
  }

  pub fn set(&mut self, key: String, val: Function) {
    self.records.insert(key, val);
  }

  pub fn resolve(&mut self, key: String) -> Option<&Function>{
    return self.records.get(&key);
  }
}