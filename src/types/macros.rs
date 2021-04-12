use std::env;

use crate::{misc::RuntimeResult, runtime::Runtime};

use super::{callable::Callable, list::List, symbol::Symbol, t_object::TObject};

#[derive(Debug, Clone, PartialEq)]
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

        let old_scope = ctx.current_scope_id;

        ctx.new_child_scope();
        for (param, arg) in self.params.iter().zip(args.into_iter()) {
            ctx.set_local(param.clone(), arg);
        }

        let expr = TObject::List(self.body.clone());
        let expanded_expr = ctx.eval(&expr)?;

        if env::var("DEBUG1").is_ok() {
            eprintln!("{:?}", expanded_expr);
        }

        ctx.set_scope(old_scope);

        ctx.eval(&expanded_expr)
    }
}
