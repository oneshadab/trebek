use std::io::{self, stdin, Write};

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

    pub fn next(&mut self) {
        let program = self.read().unwrap();
        let output = self.eval(program);
        println!("{}", output);
    }

    pub fn read(&self) -> Result<String, String> {
        let mut program = String::new();

        match stdin().read_line(&mut program) {
            Ok(_) => Ok(program),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn eval(&mut self, program: String) -> String {
        let exprs = self.parser.tokenize(&program);

        let mut out = TObject::Empty;

        for expr in exprs {
            let list = self.parser.parse(&expr);
            out = self.runtime.eval(&TObject::List(list));
        }

        format!("{}", out)
    }
}
