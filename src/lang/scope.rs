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

  pub fn resolve(&self, key: &String) -> Record {
    let default = Record::Symbol(key.clone().into());
    return self.lookup(key).unwrap_or(default);
  }

  pub fn lookup(&self, key: &String) -> Option<Record>{
    return self.records.get(key).cloned();
  }
}