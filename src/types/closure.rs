use crate::{misc::RuntimeResult, runtime::Runtime};

use super::{callable::Callable, list::List, symbol::Symbol, t_object::TObject};

#[derive(Debug, Clone)]
pub struct Closure {
    pub lexical_scope_id: usize,

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
    fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
        if self.params.len() != args.len() {
            return Err(format!("Function called with incorrect number of params!"));
        }

        let arg_vals: Vec<_> = args.iter().map(|arg| ctx.eval(&arg.clone())).collect();

        ctx.set_scope(self.lexical_scope_id);
        ctx.new_child_scope();

        for (param, arg_val) in self.params.iter().zip(arg_vals.into_iter()) {
            ctx.set_local(param.clone(), arg_val?);
        }

        let expr = TObject::List(self.body.clone());
        ctx.eval(&expr)
    }
}
