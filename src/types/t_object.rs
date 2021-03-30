use crate::memory::object_heap::ObjectId;

use super::{builtin::Builtin, closure::Closure, list::List, scope::Scope, symbol::Symbol};

#[derive(Debug, Clone)]
pub enum TObject {
    Closure(Closure),
    Builtin(Builtin),
    Symbol(Symbol),
    List(List),
    Scope(Scope),
    Empty,
}

impl std::fmt::Display for TObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TObject::Closure(_) => {
                write!(f, "[Closure]")
            }
            TObject::Builtin(_) => {
                write!(f, "[Builtin]")
            }
            TObject::Symbol(sym) => {
                write!(f, "{}", sym)
            }
            TObject::List(list) => {
                write!(f, "(")?;
                list.iter().try_for_each(|obj| write!(f, "{}", obj))?;
                write!(f, ")")
            }
            TObject::Scope(_) => {
                write!(f, "[Scope]")
            }
            TObject::Empty => {
                write!(f, "")
            }
        }
    }
}

impl TObject {
    pub fn trace(&self) -> Vec<ObjectId> {
        match self {
            TObject::Scope(scope) => {
                let mut reachable = Vec::new();

                for v in scope.obj_map.values() {
                    reachable.push(v.clone())
                }

                for v in scope.obj_stack.iter() {
                    reachable.push(v.clone());
                }

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
