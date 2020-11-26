use std::collections::HashMap;

use super::types::{record::Record, symbol::Symbol};

#[derive(Debug)]
pub struct Scope {
  records: HashMap<Symbol, Record>
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      records: HashMap::new()
    }
  }

  pub fn set(&mut self, key: Symbol, val: Record) {
    self.records.insert(key, val);
  }

  pub fn resolve(&self, key: &Symbol) -> Record {
    let default = Record::Symbol(key.clone().into());
    return self.lookup(key).unwrap_or(default);
  }

  pub fn lookup(&self, key: &Symbol) -> Option<Record>{
    return self.records.get(key).cloned();
  }
}