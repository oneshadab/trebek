use super::{builtin::Builtin, list::List, closure::Closure, symbol::Symbol};


#[derive(Debug, Clone)]
pub enum TObject {
  Closure(Closure),
  Builtin(Builtin),
  Symbol(Symbol),
  List(List),
  Empty
}
