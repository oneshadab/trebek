use crate::lang::{runtime::Runtime, types::{builtin::Builtin, record::Record}};


pub fn get_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new("+", add),
  ]
}

fn add(ctx: &mut Runtime, args: &[Record]) -> Record {
  match args {
    [arg, other_arg] => {
      let val = ctx.eval(arg);
      let other_val = ctx.eval(other_arg);

      match (val, other_val) {
        (Record::Symbol(a), Record::Symbol(b)) => {
          let i_a: i32 = a.parse().unwrap();
          let i_b: i32 = b.parse().unwrap();
          let i_result = i_a + i_b;

          return Record::Symbol(i_result.to_string());
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
