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
