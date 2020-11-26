use std::fmt;
use crate::lang::runner::Runner;

use super::{record::Record, symbol::Symbol};

type ApplyFn = fn(&mut Runner, &[Record]) -> Record;

#[derive(Clone)]
pub struct Builtin {
  pub name: &'static str,
  pub apply: ApplyFn
}

impl Builtin {
  pub fn new(name: &'static str, apply: ApplyFn) -> Builtin {
    Builtin {
      name,
      apply
    }
  }
}

impl fmt::Debug for Builtin {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct(&self.name[..]).finish()
  }
}
