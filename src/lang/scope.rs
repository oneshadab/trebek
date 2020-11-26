use std::{collections::HashMap, rc::Rc, cell::RefCell};

use super::types::{record::Record, symbol::Symbol};

#[derive(Debug)]
pub struct Scope {
  pub parent_scope_id: Option<usize>,
  pub records: HashMap<Symbol, Record>
}

impl Scope {
  pub fn new(parent_scope_id: Option<usize>) -> Scope {
    Scope {
      parent_scope_id,
      records: HashMap::new()
    }
  }

  pub fn set(&mut self, key: Symbol, val: Record) {
    self.records.insert(key, val);
  }

  pub fn lookup(&self, key: &Symbol) -> Option<Record>{
    return self.records.get(key).cloned();
  }
}