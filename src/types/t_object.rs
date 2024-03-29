use crate::memory::object_heap::ObjectId;

use super::{
    builtin::Builtin, closure::Closure, dict::Dict, list::List, macros::Macro, scope::Scope,
    string_literal::TString, symbol::Symbol,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TObject {
    Closure(Closure),
    Builtin(Builtin),
    Macro(Macro),
    Symbol(Symbol),
    String(TString),
    List(List),
    Dict(Dict),
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
            TObject::Macro(_) => {
                write!(f, "[Macro]")
            }
            TObject::Symbol(sym) => {
                write!(f, "{}", sym)
            }
            TObject::Scope(_) => {
                write!(f, "[Scope]")
            }
            TObject::Empty => {
                write!(f, "")
            }
            TObject::String(s) => {
                write!(f, "{}", s)
            }
            TObject::List(list) => {
                let out = list
                    .iter()
                    .map(|obj| obj.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                write!(f, "({})", out)
            }

            TObject::Dict(dict) => {
                let out = dict
                    .iter()
                    .map(|(k, v)| format!("{} {}", k, v))
                    .collect::<Vec<String>>()
                    .join(" ");

                write!(f, "{{{}}}", out)
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
