use std::{error::Error, io::{self, Write, stdin}};

use crate::runner::Runner;

pub fn repl() {
  let mut runner = Runner::new();

  loop {
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let program = read().unwrap();
    let output = runner.eval(program);
    println!("{}", output);
  }
}

pub fn read() -> Result<String, String> {
  let mut program = String::new();

  match stdin().read_line(&mut program) {
    Ok(_) => { Ok(program) }
    Err(e) => { Err(e.to_string()) }
  }
}
