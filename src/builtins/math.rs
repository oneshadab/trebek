use crate::{runtime::Runtime, types::{builtin::Builtin, t_object::TObject}};


pub fn get_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new("+", add),
    Builtin::new("-", subtract),
    Builtin::new("*", multiply),
    Builtin::new("/", divide),
  ]
}

macro_rules! create_math_function {
  ($func_name: ident, $operation: tt) => {
    fn $func_name(ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
      match &args[..] {
        [arg, other_arg] => {
          let val = ctx.eval(arg);
          let other_val = ctx.eval(other_arg);

          match (val, other_val) {
            (TObject::Symbol(a), TObject::Symbol(b)) => {
              let i_a: i32 = a.parse().unwrap();
              let i_b: i32 = b.parse().unwrap();
              let i_result = i_a $operation i_b;

              return TObject::Symbol(i_result.to_string());
            }
            _ => {
              panic!("'add' called with incorrect params")
            }
          }
        }
        _ => {
          panic!("'add' called with incorrect number of args")
        }
      }
    }
  };
}

create_math_function!(add, +);
create_math_function!(subtract, -);
create_math_function!(multiply, *);
create_math_function!(divide, /);
