use super::{parser::Parser, runtime::Runtime, types::t_object::TObject};

pub struct Runner {
  pub parser: Parser,
  pub runtime: Runtime
}

impl Runner {
  pub fn new() -> Runner {
    Runner {
      parser: Parser::new(),
      runtime: Runtime::new()
    }
  }

  pub fn run(&mut self, program: String) -> String {
    let exprs = self.parser.tokenize(&program);

    let mut out = TObject::Empty;

    for expr in exprs {
      let list = self.parser.parse(&expr);
      out = self.runtime.eval(&TObject::List(list));
    }

    format!("{:?}", out)
  }
}
