use std::{collections::HashMap};

use super::types::{tobject::TObject, symbol::Symbol};

#[derive(Debug)]
pub struct Scope {
  pub parent_scope_id: Option<usize>,
  pub objs: HashMap<Symbol, TObject>
}

impl Scope {
  pub fn new(parent_scope_id: Option<usize>) -> Scope {
    Scope {
      parent_scope_id,
      objs: HashMap::new()
    }
  }

  pub fn set(&mut self, key: Symbol, val: TObject) {
    self.objs.insert(key, val);
  }

  pub fn lookup(&self, key: &Symbol) -> Option<TObject>{
    return self.objs.get(key).cloned();
  }
}
