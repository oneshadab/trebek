use crate::lang::{runtime::Runtime, types::{builtin::Builtin, record::Record}};


pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("def", def),
  ]
}

pub fn def(ctx: &mut Runtime, args: &[Record]) -> Record {
  match args {
    [Record::Symbol(symbol), val] => {
      ctx.set_global(symbol.clone(), val.clone());

      return Record::Empty;
    }
    _ => {
      panic!("'def' called with incorrect number of args")
    }
  }
}
