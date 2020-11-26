use crate::lang::runner::Runner;

use super::{expression::Expression, record::Record, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct Function {
  params: Vec<Symbol>,
  body: Expression
}

impl Function {
  pub fn apply(&self, ctx: &mut Runner, args: Vec<Record>) -> Record {
    if self.params.len() != args.len() {
      panic!("Function called with incorrect number of params!")
    }

    for (i, _) in self.params.iter().enumerate() {
      ctx.set_local(self.params[i].clone(), args[i].clone());
    }

    let expr = Record::Expression(self.body.clone());
    ctx.eval(&expr)
  }
}