use std::convert::TryInto;

use crate::{runtime::{Runtime, RuntimeResult}, types::closure::Closure, types::{builtin::Builtin, list::List, symbol::Symbol, t_object::TObject}};

pub fn get_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("fn", create_function),
        Builtin::new("defn", define_function),
    ]
}

fn create_function(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [TObject::List(params_expr), TObject::List(body_expr)] => {
            let func = init_function(ctx, params_expr, body_expr)?;
            Ok(TObject::Closure(func))
        }
        _ => {
            Err(format!("'print' called with incorrect number of args"))
        }
    }
}

fn define_function(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [TObject::Symbol(symbol), TObject::List(params_expr), TObject::List(body_expr)] => {
            let func = init_function(ctx, params_expr, body_expr)?;
            ctx.set_global(symbol.clone(), TObject::Closure(func));

            Ok(TObject::Empty)
        }
        _ => {
            Err(format!("'defn' called with incorrect number of args"))
        }
    }
}

fn init_function(ctx: &mut Runtime, params: &List, body: &List) -> RuntimeResult<Closure> {
    let unwrapped_params= params
        .into_iter()
        .map(|p| match p {
            TObject::Symbol(s) => {
                Ok(s.clone())
            }
            other => {
                Err(format!("`{}` param must be a symbol!", other))
            }
        })
        .collect::<RuntimeResult<Vec<Symbol>>>()?;

    Ok(Closure::new(ctx, unwrapped_params, body.clone()))
}
