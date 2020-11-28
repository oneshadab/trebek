use crate::lang::{runtime::Runtime, types::{builtin::Builtin, record::Record}};


pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("def", define),
  ]
}

pub fn define(ctx: &mut Runtime, args: &[Record]) -> Record {
  match args {
    [Record::Symbol(symbol), val] => {
      ctx.set_global(symbol.clone(), val.clone());

      Record::Empty
    }
    _ => {
      panic!("'def' called with incorrect number of args")
    }
  }
}
