use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
    types::{builtin::Builtin, t_object::TObject},
};

pub fn get_builtins() -> Vec<Builtin> {
    vec![Builtin::new("str", make_string)]
}

fn make_string(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    let concated_string = args
        .iter()
        .map(|arg| -> RuntimeResult<String> {
            let v = ctx.eval(arg)?;
            Ok(v.to_string())
        })
        .collect::<RuntimeResult<String>>()?;

    Ok(TObject::Symbol(concated_string))
}
