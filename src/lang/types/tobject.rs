use super::{builtin::Builtin, expression::Expression, closure::Closure, symbol::Symbol};


#[derive(Debug, Clone)]
pub enum TObject {
  Closure(Closure),
  Builtin(Builtin),
  Symbol(Symbol),
  Expression(Expression),
  Empty
}
