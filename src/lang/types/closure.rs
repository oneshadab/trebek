use crate::lang::runtime::Runtime;

use super::{callable::Callable, list::List, t_object::TObject, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct Closure {
  lexical_scope_id: usize,

  params: Vec<Symbol>,
  body: List,
}

impl Closure {
  pub fn new(ctx: &Runtime, params: Vec<Symbol>, body: List) -> Closure {
    Closure {
      lexical_scope_id: ctx.current_scope_id,

      params,
      body,
    }
  }
}

impl Callable for Closure {
  fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
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

    eprintln!("DBG: {:?}", ctx.eval(&TObject::Symbol("n".into())));
    let expr = TObject::List(self.body.clone());
    ctx.eval(&expr)
  }
}
