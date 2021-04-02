use std::io::{stdin, stdout, Write};

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
        let program = self.read_expr().unwrap();
        let output = match self.eval(program) {
            Ok(out) => out,
            Err(e) => {
                format!("Error: {}", e.to_string())
            }
        };
        println!("{}", output);
    }

    pub fn read_expr(&self) -> Result<String, String> {
        let mut lines: Vec<String> = vec![];

        let mut depth = 0;

        print!(">>> ");

        loop {
            stdout().flush().ok().expect("Could not flush stdout");

            let mut buffer = String::new();
            if let Err(e) = stdin().read_line(&mut buffer) {
                return Err(e.to_string());
            }

            for ch in buffer.chars() {
                if ch == '(' {
                    depth += 1;
                }
                if ch == ')' {
                    depth -= 1;
                }
            }

            lines.push(buffer);

            if depth == 0 {
                break;
            }

            print!("... ")
        }

        Ok(lines.join(""))
    }

    pub fn eval(&mut self, program: String) -> RuntimeResult<String> {
        let exprs = self.parser.parse(&program)?;

        let mut out = TObject::Empty;
        for expr in exprs {
            out = self.runtime.eval(&expr)?;
        }

        Ok(format!("{}", out))
    }
}
