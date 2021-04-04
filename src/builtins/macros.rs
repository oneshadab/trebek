use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
    types::macros::Macro,
    types::symbol::Symbol,
    types::{builtin::Builtin, t_object::TObject},
};

pub fn get_builtins() -> Vec<Builtin> {
    vec![Builtin::new("defmacro", define_macro)]
}

fn define_macro(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [TObject::Symbol(sym), TObject::List(params_expr), TObject::List(body_expr)] => {
            let params = params_expr
                .into_iter()
                .map(|p| match p {
                    TObject::Symbol(s) => Ok(s.clone()),
                    _ => Err(format!("param not symbol")),
                })
                .collect::<RuntimeResult<Vec<Symbol>>>()?;

            let m = Macro::new(params, body_expr.clone());
            ctx.set_global(sym.clone(), TObject::Macro(m));
            Ok(TObject::Empty)
        }
        _ => Err(format!("'defmacro' called with incorrect number of args")),
    }
}
