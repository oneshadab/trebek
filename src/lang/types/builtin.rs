use std::fmt;
use crate::lang::runtime::Runtime;

use super::{callable::Callable, t_object::TObject};

type Func = fn(&mut Runtime, Vec<TObject>) -> TObject;

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
  fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
    (self.func)(ctx, args)
  }
}

impl fmt::Debug for Builtin {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct(&self.name[..]).finish()
  }
}
