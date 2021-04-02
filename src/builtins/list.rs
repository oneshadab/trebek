use crate::{misc::RuntimeResult, runtime::Runtime, types::{builtin::Builtin, list::List, t_object::TObject}};

pub fn get_builtins() -> Vec<Builtin> {
  vec![Builtin::new("list", make_list)]
}

fn make_list(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
  let evaled_args = args
      .iter()
      .map(|arg| ctx.eval(arg))
      .collect::<RuntimeResult<List>>()?;

  Ok(TObject::List(evaled_args))
}
