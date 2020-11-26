use std::fmt;
use super::{runner::Runner};

pub type Builtin = fn(&mut Runner, &[Record]) -> Record;
pub type Symbol = String;
pub type Expression = String;

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

#[derive(Debug, Clone)]
pub enum Record {
  Function(Function),
  Builtin(Builtin),
  Symbol(Symbol),
  Expression(Expression),
  Empty
}
