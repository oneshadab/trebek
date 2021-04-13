use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
    types::{builtin::Builtin, t_object::TObject},
    types::callable::Callable,
};

pub fn get_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("dict", make_dict),
        Builtin::new("dict-get", dict_get),
    ]
}

fn make_dict(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    let mut keys: Vec<String> = Vec::new();
    let mut vals: Vec<TObject> = Vec::new();

    for (i, arg) in args.iter().enumerate() {
        let obj = ctx.eval(arg)?;

        if i % 2 == 0 {
            // Only allow string keys for now
            let s = match obj {
                TObject::Symbol(s) => Ok(s),
                TObject::String(s) => Ok(s),
                v => Err(format!("`{}` cannot be used as a key", v)),
            }?;

            keys.push(s);
        } else {
            vals.push(obj);
        }
    }

    let dict = keys.into_iter().zip(vals.into_iter()).collect();

    Ok(TObject::Dict(dict))
}

fn dict_get(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [dict_obj, obj] => {
            let dict = ctx.eval(dict_obj)?;

            match dict {
                TObject::Dict(dict) => {
                    dict.call(ctx, vec![obj.clone()])
                }
                _ => Err(format!("'dict-get' called with incorrect number of args")),
            }
        }
        _ => Err(format!("'dict-get' called with incorrect number of args")),
    }
}
