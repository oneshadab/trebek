use std::io::{BufRead, Write};
use crate::lang::{runtime::Runtime, types::{builtin::Builtin, record::Record}};


pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("scan", scan),
    Builtin::new("print", print),
  ]
}

fn scan(ctx: &mut Runtime, args: Vec<Record>) -> Record {
  match &args[..] {
    [] => {
      let mut word = String::new();

      ctx.reader.read_line(&mut word).unwrap();
      word.pop(); // Remove trailing newline

      Record::Symbol(word)
    }
    _ => {
      panic!("'scan' called with incorrect number of args")
    }
  }
}

fn print(ctx: &mut Runtime, args: Vec<Record>) -> Record {
  match &args[..] {
    [symbol] => {
      let val = ctx.eval(symbol);
      writeln!(&mut ctx.writer, "{:?}", val).unwrap();
      Record::Empty
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}
