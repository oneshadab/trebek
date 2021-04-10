use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
    types::{builtin::Builtin, t_object::TObject},
};

pub fn get_builtins() -> Vec<Builtin> {
    vec![Builtin::new("quote", quote), Builtin::new("eval", eval)]
}

fn quote(_ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    if args.len() != 1 {
        return Err(format!("'quote' called with incorrect args"));
    }

    Ok(args[0].clone())
}

fn eval(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [obj] => {
            let expanded_obj = ctx.eval(obj)?;
            ctx.eval(&expanded_obj)
        }
        _ => Err(format!("'eval' called with incorrect args")),
    }
}
