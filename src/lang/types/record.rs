use super::{builtin::Builtin, expression::Expression, function::Function, symbol::Symbol};


#[derive(Debug, Clone)]
pub enum Record {
  Function(Function),
  Builtin(Builtin),
  Symbol(Symbol),
  Expression(Expression),
  Empty
}
