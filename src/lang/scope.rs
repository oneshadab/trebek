use std::{collections::HashMap};

use super::{memory::object_heap::ObjectId, types::{ symbol::Symbol}};

#[derive(Debug)]
pub struct Scope {
  pub parent_scope_id: Option<usize>,
  pub objs: HashMap<Symbol, ObjectId>
}

impl Scope {
  pub fn new(parent_scope_id: Option<usize>) -> Scope {
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
