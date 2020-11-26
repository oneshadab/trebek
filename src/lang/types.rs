use std::fmt;
use super::{runner::Runner};

pub type Function = fn(&mut Runner, &[Record]) -> Record;
pub type Symbol = String;
pub type Expression = String;

pub enum Record {
  Function(Function),
  Symbol(Symbol),
  Expression(Expression),
  Empty
}

impl fmt::Debug for Record {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Record::Function(func) => {
        f.debug_struct("[Function]").finish()
      }
      Record::Symbol(symbol) => {
        f.debug_struct(symbol).finish()
      }
      Record::Expression(expr) => {
        f.debug_struct(expr).finish()
      }
      Record::Empty => {
        f.debug_struct("").finish()
      }
    }
  }
}

impl Clone for Record {
  fn clone(&self) -> Self {
      match self {
          Record::Function(func) => {
            Record::Function(*func)
          }
          Record::Symbol(symbol) => {
            Record::Symbol(symbol.into())
          }
          Record::Expression(expr) => {
            Record::Expression(expr.into())
          }
          Record::Empty => {
            Record::Empty
          }
      }
  }
}