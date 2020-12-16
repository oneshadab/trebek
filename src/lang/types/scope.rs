use std::{collections::HashMap};

use crate::lang::memory::object_heap::ObjectId;

use super::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Scope {
  pub parent_scope_id: Option<ObjectId>,
  pub objs: HashMap<Symbol, ObjectId>
}

impl Scope {
  pub fn new(parent_scope_id: Option<ObjectId>) -> Scope {
    Scope {
      parent_scope_id,
      objs: HashMap::new()
    }
  }

  pub fn set(&mut self, key: Symbol, val: ObjectId) {
    self.objs.insert(key, val);
  }

  pub fn lookup(&self, key: &Symbol) -> Option<ObjectId>{
    return self.objs.get(key).cloned();
  }
}
