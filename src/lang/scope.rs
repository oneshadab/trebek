use std::collections::HashMap;
use super::types::{Record, Symbol};


pub struct Scope {
  records: HashMap<Symbol, Record>
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      records: HashMap::new()
    }
  }

  pub fn set(&mut self, key: String, val: Record) {
    self.records.insert(key, val);
  }

  pub fn resolve(&mut self, key: &String) -> Option<&Record>{
    return self.records.get(key);
  }
}