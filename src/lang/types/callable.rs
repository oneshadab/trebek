use crate::lang::runtime::Runtime;

use super::record::Record;

pub trait Callable {
  fn call(&self, ctx: &mut Runtime, args: Vec<Record>) -> Record;
}
