use crate::{
  constants::{FALSE, TRUE},
  misc::RuntimeResult,
  runtime::Runtime,
  types::builtin::Builtin,
  types::t_object::TObject,
};

pub fn get_builtins() -> Vec<Builtin> {
  vec![
      Builtin::new("if", if_cond),
      Builtin::new("do", execute_seqentially),
  ]
}

fn if_cond(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
  match &args[..] {
      [cond_expr, true_expr, false_expr] => {
          let cond_res = ctx.eval(cond_expr)?;

          let res = match cond_res {
              TObject::Symbol(symbol) => match symbol {
                  s if s == TRUE => ctx.eval(true_expr),
                  s if s == FALSE => ctx.eval(false_expr),
                  _ => Err(format!("{:?} is not true/false!", symbol)),
              },
              other => Err(format!("{:?} is not a boolean!", other)),
          };

          res
      }
      _ => Err(format!("'print' called with incorrect number of args")),
  }
}

fn execute_seqentially(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
  let mut out = TObject::Empty;
  for arg in args.iter() {
    out = ctx.eval(arg)?;
  }
  Ok(out)
}
