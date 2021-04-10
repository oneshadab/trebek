use crate::{
    constants::{FALSE, TRUE},
    misc::RuntimeResult,
    runtime::Runtime,
    types::builtin::Builtin,
    types::t_object::TObject,
};

pub fn get_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("=", is_equal),
        Builtin::new("<", is_less),
    ]
}

fn is_equal(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [left_expr, right_expr] => {
            let left = ctx.eval(left_expr)?;
            let right = ctx.eval(right_expr)?;

            let res = match (&left, &right) {
                (TObject::Symbol(lhs), TObject::Symbol(rhs)) => {
                    if lhs == rhs {
                        TObject::Symbol(TRUE.into())
                    } else {
                        TObject::Symbol(FALSE.into())
                    }
                }
                _ => Err(format!("Cannot compare {:?} and {:?}", left, right))?,
            };

            Ok(res)
        }
        _ => Err(format!("'print' called with incorrect number of args")),
    }
}

fn is_less(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [left_expr, right_expr] => {
            let left = ctx.eval(left_expr)?;
            let right = ctx.eval(right_expr)?;

            let res = match (&left, &right) {
                (TObject::Symbol(lhs), TObject::Symbol(rhs)) => {
                    if lhs < rhs {
                        TObject::Symbol(TRUE.into())
                    } else {
                        TObject::Symbol(FALSE.into())
                    }
                }
                _ => Err(format!("Cannot compare {:?} and {:?}", left, right))?,
            };

            Ok(res)
        }
        _ => Err(format!("'print' called with incorrect number of args")),
    }
}
