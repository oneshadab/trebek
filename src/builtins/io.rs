use crate::{runtime::{Runtime, RuntimeResult}, types::{builtin::Builtin, t_object::TObject}};
use std::io::{BufRead, Write};

pub fn get_builtins() -> Vec<Builtin> {
    vec![Builtin::new("scan", scan), Builtin::new("print", print)]
}

fn scan(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [] => {
            let mut word = String::new();

            ctx.reader.read_line(&mut word).unwrap();
            word.pop(); // Remove trailing newline

            Ok(TObject::Symbol(word))
        }
        _ => {
            panic!("'scan' called with incorrect number of args")
        }
    }
}

fn print(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [symbol] => {
            let val = ctx.eval(symbol)?;
            writeln!(&mut ctx.writer, "{}", val).unwrap();
            Ok(TObject::Empty)
        }
        _ => {
            panic!("'print' called with incorrect number of args")
        }
    }
}
