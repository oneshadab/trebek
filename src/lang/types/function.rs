use crate::lang::runtime::Runtime;

use super::{callable::Callable, expression::Expression, record::Record, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct Function {
  lexical_scope_id: usize,

  params: Vec<Symbol>,
  body: Expression,
}

impl Function {
  pub fn new(ctx: &Runtime, params: Vec<Symbol>, body: Expression) -> Function {
    Function {
      lexical_scope_id: ctx.current_scope_id,

      params,
      body,
    }
  }
}

impl Callable for Function {
  fn call(&self, ctx: &mut Runtime, args: Vec<Record>) -> Record {
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
