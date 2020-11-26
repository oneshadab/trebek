use super::{types::Record, runner::Runner};


pub fn add(ctx: &mut Runner, args: &[Record]) -> Record {
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

pub fn def(ctx: &mut Runner, args: &[Record]) -> Record {
  match args {
    [Record::Symbol(symbol), val] => {
      ctx.root_scope.set(symbol.clone(), val.clone());

      return Record::Empty;
    }
    _ => {
      panic!("'def' called with incorrect number of args")
    }
  }
}

pub fn print(ctx: &mut Runner, args: &[Record]) -> Record {
  match args {
    [symbol] => {
      let val = ctx.eval(symbol);
      println!("{:?}", val);
      Record::Empty
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}