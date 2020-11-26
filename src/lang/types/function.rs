use crate::lang::runner::Runner;

use super::{expression::Expression, record::Record, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct Function {
  params: Vec<Symbol>,
  body: Expression
}

impl Function {
  pub fn new(params: Vec<Symbol>, body: Expression) -> Function {
    Function {
      params,
      body
    }
  }

  pub fn apply(&self, ctx: &mut Runner, args: Vec<Record>) -> Record {
    if self.params.len() != args.len() {
      panic!("Function called with incorrect number of params!")
    }

    for (i, _) in self.params.iter().enumerate() {
      let arg_val = ctx.eval(&args[i].clone());
      ctx.set_local(self.params[i].clone(), arg_val);
    }

    let expr = Record::Expression(self.body.clone());
    ctx.eval(&expr)
  }
}