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
  }

  pub fn eval(&mut self, expr: String) -> Record {
    let tokens = Parser::new().tokenize(expr);

    let func_name = &tokens[0];
    let args: Vec<String> = tokens[1..].into();

    match self.root_scope.resolve(func_name) {
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