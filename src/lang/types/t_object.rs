use crate::lang::memory::object_heap::ObjectId;

use super::{builtin::Builtin, closure::Closure, list::List, scope::Scope, symbol::Symbol};


#[derive(Debug, Clone)]
pub enum TObject {
  Closure(Closure),
  Builtin(Builtin),
  Symbol(Symbol),
  List(List),
  Scope(Scope),
  Empty
}

impl TObject {
  pub fn trace(&self) -> Vec<ObjectId> {
    match self {
        TObject::Scope(scope ) => {
          let mut reachable: Vec<ObjectId> = scope.objs
            .values()
            .map(|v| v.clone())
            .collect();

          if let Some(scope_id) = scope.parent_scope_id {
            reachable.push(scope_id);
          }

          reachable
        }
        TObject::Closure(func) => {
          vec![func.lexical_scope_id]
        }
        _ => {
          vec![]
        }
    }
  }
}
