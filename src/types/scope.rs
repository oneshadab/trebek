use std::collections::HashMap;

use crate::memory::object_heap::ObjectId;

use super::symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub parent_scope_id: Option<ObjectId>,
    pub obj_map: HashMap<Symbol, ObjectId>,
    pub obj_stack: Vec<ObjectId>,
}

impl Scope {
    pub fn new(parent_scope_id: Option<ObjectId>) -> Scope {
        Scope {
            parent_scope_id,
            obj_map: HashMap::new(),
            obj_stack: Vec::new(),
        }
    }

    pub fn set(&mut self, key: Symbol, val: ObjectId) {
        self.obj_map.insert(key, val);
    }

    pub fn lookup(&self, key: &Symbol) -> Option<ObjectId> {
        return self.obj_map.get(key).cloned();
    }

    pub fn push(&mut self, val: ObjectId) {
        self.obj_stack.push(val);
    }

    pub fn pop(&mut self) -> ObjectId {
        self.obj_stack.pop().unwrap()
    }
}
