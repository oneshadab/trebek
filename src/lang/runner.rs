use super::{types::Record, parser::Parser, scope::Scope, builtins};
pub struct Runner {
  root_scope: Scope
}

impl Runner {
  pub fn new() -> Runner{
    let mut runner =  Runner {
      root_scope: Scope::new()
    };

    runner.init_builtins();

    return runner;
  }

  fn init_builtins(&mut self) {
    self.root_scope.set(String::from("+"), Record::Function(builtins::add));
    self.root_scope.set(String::from("def"), Record::Function(builtins::def));
    self.root_scope.set(String::from("print"), Record::Function(builtins::print));
  }

  pub fn run(&mut self, program: String) -> Record {
    let exprs = Parser::new().tokenize(&program);

    let mut out = Record::Empty;
    for expr in exprs {
      println!("{}", expr);
      out = self.eval(expr);
    }
    return out;
  }

  pub fn eval(&mut self, expr: String) -> Record {
    let parser = Parser::new();
    let tokens = parser.tokenize(&parser.trim(&expr));

    let func_name = &tokens[0];
    let args: Vec<String> = tokens[1..].into();

    match self.root_scope.lookup(func_name) {
      Some(record) => {
        match record {
          Record::Function(func) => {
            func(&mut self.root_scope, args)
          },
          Record::Symbol(symbol) => {
            panic!("Symbol is not callable")
          },
          Record::Expression(expr) => {
            panic!("Expression calling isn't supported (yet)")
          },
          Record::Empty => {
            Record::Empty
          }
        }
      }
      None => {
        panic!("Function not found!")
      }
    }
  }
}