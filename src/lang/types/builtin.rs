use std::fmt;
use crate::lang::runtime::Runtime;

use super::{record::Record, symbol::Symbol};

type Func = fn(&mut Runtime, &[Record]) -> Record;

#[derive(Clone)]
pub struct Builtin {
  pub name: &'static str,
  pub func: Func
}

impl Builtin {
  pub fn new(name: &'static str, func: Func) -> Builtin {
    Builtin {
      name,
      func
    }
  }

  pub fn apply(&self, ctx: &mut Runtime, args: &[Record]) -> Record {
    (self.func)(ctx, args)
  }
}

impl fmt::Debug for Builtin {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct(&self.name[..]).finish()
  }
}
