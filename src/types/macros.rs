use std::env;

use crate::{misc::RuntimeResult, runtime::Runtime};

use super::{callable::Callable, list::List, symbol::Symbol, t_object::TObject};

#[derive(Debug, Clone)]
pub struct Macro {
    params: Vec<Symbol>,
    body: List, // Todo: Change from List -> TObject
}

impl Macro {
    pub fn new(params: Vec<Symbol>, body: List) -> Macro {
        Macro { params, body }
    }
}

impl Callable for Macro {
    fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
        if self.params.len() != args.len() {
            return Err(format!("Macro called with incorrect number of params!"));
        }

        ctx.new_child_scope();
        for (param, arg) in self.params.iter().zip(args.into_iter()) {
            ctx.set_local(param.clone(), arg);
        }

        let expr = TObject::List(self.body.clone());
        let expanded_expr = ctx.eval(&expr)?;

        if env::var("DEBUG").is_ok() {
            eprintln!("{:?}", expanded_expr);
        }

        ctx.eval(&expanded_expr)
    }
}
