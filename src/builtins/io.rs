use crate::to_runtime_result;
use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
    types::{builtin::Builtin, t_object::TObject},
};
use std::{convert::TryInto, io::{BufRead, Write}};

pub fn get_builtins() -> Vec<Builtin> {
    vec![Builtin::new("scan", scan), Builtin::new("print", print)]
}

fn scan(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [] => {
            let mut word = String::new();

            to_runtime_result!(ctx.reader.read_line(&mut word))?;

            word.pop(); // Remove trailing newline

            Ok(TObject::Symbol(word))
        }
        _ => Err(format!("'scan' called with incorrect number of args")),
    }
}

fn print(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    let output = args
        .iter()
        .map(|arg| ctx.eval(arg))
        .collect::<RuntimeResult<Vec<TObject>>>()?
        .iter()
        .map(|obj| format!("{}", &obj))
        .collect::<Vec<String>>()
        .join("");

    to_runtime_result!(writeln!(&mut ctx.writer, "{}", output))?;
    Ok(TObject::Empty)
}
