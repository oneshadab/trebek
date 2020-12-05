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

    let arg_vals: Vec<_> = args.iter()
      .map(|arg| { ctx.eval(&arg.clone()) })
      .collect();

    ctx.restore_scope(self.lexical_scope_id);
    ctx.new_child_scope();

    for (param, arg_val) in self.params.iter().zip(arg_vals.into_iter()) {
      ctx.set_local(param.clone(), arg_val);
    }

    eprintln!("DBG: {:?}", ctx.eval(&Record::Symbol("n".into())));
    let expr = Record::Expression(self.body.clone());
    ctx.eval(&expr)
  }
}
