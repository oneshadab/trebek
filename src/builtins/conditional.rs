use crate::{types::builtin::Builtin, constants::{FALSE, TRUE}, runtime::Runtime, types::t_object::TObject};

pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("if", cond_if),
    Builtin::new("=", is_equal),
    Builtin::new("<", is_less),
  ]
}

fn cond_if(ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
  match &args[..] {
    [
      cond_expr,
      true_expr,
      false_expr
    ] => {
      let result = ctx.eval(cond_expr);

      match result {
        TObject::Symbol(symbol) => {
          match symbol {
            s if s == TRUE => { ctx.eval(true_expr) }
            s if s == FALSE => { ctx.eval(false_expr) }
            _ => panic!("{:?} is not true/false!", symbol)
          }
        }
        other => { panic!("{:?} is not a boolean!", other) }
      }
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}

fn is_equal(ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
  match &args[..] {
    [
      left_expr,
      right_expr,
    ] => {
      let left = ctx.eval(left_expr);
      let right = ctx.eval(right_expr);

      match (&left, &right) {
        (TObject::Symbol(lhs), TObject::Symbol(rhs)) => {
          if lhs == rhs {
            TObject::Symbol(TRUE.into())
          }
          else {
            TObject::Symbol(FALSE.into())
          }
          }
        _ => { panic!("Cannot compare {:?} and {:?}", left, right) }
      }
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}

fn is_less(ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
  match &args[..] {
    [
      left_expr,
      right_expr,
    ] => {
      let left = ctx.eval(left_expr);
      let right = ctx.eval(right_expr);

      match (&left, &right) {
        (TObject::Symbol(lhs), TObject::Symbol(rhs)) => {
          if lhs < rhs {
            TObject::Symbol(TRUE.into())
          }
          else {
            TObject::Symbol(FALSE.into())
          }
          }
        _ => { panic!("Cannot compare {:?} and {:?}", left, right) }
      }
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}
