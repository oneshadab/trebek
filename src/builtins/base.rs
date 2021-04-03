use crate::{
  misc::RuntimeResult,
  runtime::Runtime,
  types::{builtin::Builtin, t_object::TObject},
};

pub fn get_builtins() -> Vec<Builtin> {
  vec![Builtin::new("quote", quote)]
}

fn quote(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
  if args.len() != 1 {
    return Err(format!("'quote' called with incorrect args"));
  }

  Ok(args[0].clone())
}
