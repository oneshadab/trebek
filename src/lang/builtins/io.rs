use crate::lang::{runtime::Runtime, types::{builtin::Builtin, record::Record}};

pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("print", print),
  ]
}


pub fn print(ctx: &mut Runtime, args: &[Record]) -> Record {
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