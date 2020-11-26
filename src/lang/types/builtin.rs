use std::fmt;
use crate::lang::runner::Runner;

use super::{record::Record, symbol::Symbol};

type Func = fn(&mut Runner, &[Record]) -> Record;

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

  pub fn apply(&self, ctx: &mut Runner, args: &[Record]) -> Record {
    (self.func)(ctx, args)
  }
}

impl fmt::Debug for Builtin {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct(&self.name[..]).finish()
  }
}
