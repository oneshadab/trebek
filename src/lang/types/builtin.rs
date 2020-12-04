use std::fmt;
use crate::lang::runtime::Runtime;

use super::{callable::Callable, record::Record};

type Func = fn(&mut Runtime, Vec<Record>) -> Record;

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
}

impl Callable for Builtin {
  fn call(&self, ctx: &mut Runtime, args: Vec<Record>) -> Record {
    (self.func)(ctx, args)
  }
}

impl fmt::Debug for Builtin {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct(&self.name[..]).finish()
  }
}
