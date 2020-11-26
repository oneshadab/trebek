use std::{collections::HashMap, rc::Rc};

use super::types::{record::Record, symbol::Symbol};

#[derive(Debug)]
pub struct Scope {
  pub parent_scope: Option<Rc<Scope>>,
  pub records: HashMap<Symbol, Record>
}

impl Scope {
  pub fn new(parent_scope: Option<Rc<Scope>>) -> Scope {
    Scope {
      parent_scope,
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