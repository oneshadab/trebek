use std::{
    io::{stdout, Write},
    process,
};

use rustyline::{error::ReadlineError, Editor};

use crate::misc::RuntimeResult;

use super::{parser::Parser, runtime::Runtime, types::t_object::TObject};

pub struct Repl {
    pub parser: Parser,
    pub runtime: Runtime,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            parser: Parser::new(),
            runtime: Runtime::new(),
        }
    }

    pub fn prompt(&mut self) {
        let program = self.read().unwrap();
        let output = match self.eval(program) {
            Ok(out) => out,
            Err(e) => {
                format!("Error: {}", e.to_string())
            }
        };
        println!("{}", output);
    }

    pub fn read(&self) -> Result<String, String> {
        let mut lines: Vec<String> = vec![];

        let mut depth = 0;

        let mut rl = Editor::<()>::new();

        loop {
            stdout().flush().ok().expect("Could not flush stdout");

            let prompt = if lines.len() == 0 { ">>> " } else { "... " };

            let line = match rl.readline(prompt) {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) => {
                    process::exit(0);
                }
                Err(ReadlineError::Eof) => {
                    process::exit(0);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            };

            for ch in line.chars() {
                if ch == '(' {
                    depth += 1;
                }
                if ch == ')' {
                    depth -= 1;
                }
            }

            rl.add_history_entry(line.as_str());
            lines.push(line);

            if depth == 0 {
                break;
            }
        }

        Ok(lines.join("\n"))
    }

    pub fn eval(&mut self, program: String) -> RuntimeResult<String> {
        let exprs = self.parser.parse(&program)?;

        let mut out = TObject::Empty;
        for expr in exprs {
            out = self.runtime.eval(&expr)?;
            self.runtime.writer.flush().ok().ok_or("Failed to flush to stdout")?;
        }

        Ok(format!("{}", out))
    }
}
