use crate::lang::runtime::Runtime;

use super::tobject::TObject;

pub trait Callable {
  fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> TObject;
}
