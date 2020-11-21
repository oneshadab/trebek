use std::collections::HashMap;

pub struct Scope {
  records: HashMap<String, String>
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      records: HashMap::new()
    }
  }

  pub fn set(&mut self, key: String, val: String) {
    self.records.insert(key, val);
  }

  pub fn resolve(&mut self, key: String) -> Option<&String>{
    return self.records.get(&key);
  }
}