use crate::lang::{types::builtin::Builtin, constants::{FALSE, TRUE}, runtime::Runtime, types::record::Record};

pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("if", cond_if),
    Builtin::new("=", is_equal),
  ]
}

fn cond_if(ctx: &mut Runtime, args: &[Record]) -> Record {
match args {
    [
      cond_expr,
      true_expr,
      false_expr
    ] => {
      let result = ctx.eval(cond_expr);

      match result {
        Record::Symbol(symbol) => {
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

fn is_equal(ctx: &mut Runtime, args: &[Record]) -> Record {
  match args {
    [
      left_expr,
      right_expr,
    ] => {
      let left = ctx.eval(left_expr);
      let right = ctx.eval(right_expr);

      match (&left, &right) {
        (Record::Symbol(lhs), Record::Symbol(rhs)) => {
          if lhs == rhs {
            Record::Symbol(TRUE.into())
          }
          else {
            Record::Symbol(FALSE.into())
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
