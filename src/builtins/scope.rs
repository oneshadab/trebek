use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
    types::{builtin::Builtin, t_object::TObject},
};

pub fn get_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("def", define),
        Builtin::new("let", let_new),
        Builtin::new("import", import),
    ]
}

fn define(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [TObject::Symbol(symbol), expr] => {
            let val = ctx.eval(expr)?;

            ctx.set_global(symbol.clone(), val.clone());

            Ok(val)
        }
        _ => Err(format!("'def' called with incorrect args")),
    }
}

fn let_new(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [TObject::List(assignment_expr), body] => {
            let keys_and_vals = assignment_expr;

            let keys: Vec<_> = keys_and_vals.iter().step_by(2).collect();
            let vals: Vec<_> = keys_and_vals.iter().skip(1).step_by(2).collect();

            if keys.len() != vals.len() {
                Err(format!("Number of symbols and vals don't match"))?;
            }

            for (key, val) in keys.into_iter().zip(vals.into_iter()) {
                let lhs = match key {
                    TObject::Symbol(symbol) => symbol.clone(),
                    other => Err(format!("{} is not a symbol", other))?,
                };

                let rhs = ctx.eval(val)?;

                ctx.set_local(lhs, rhs);
            }

            ctx.eval(body)
        }
        _ => Err(format!("'let' called with incorrent args")),
    }
}

fn import(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [file_path] => {
            match ctx.eval(file_path)? {
                TObject::String(path) => ctx.import(path),
                other => Err(format!("cannot import `{}`", other)),
            }
        }
        _ => Err(format!("'let' called with incorrent args")),
    }
}
