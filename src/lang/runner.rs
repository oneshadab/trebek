use super::{parser::Parser, scope::Scope, builtins};
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
    self.root_scope.set("+".into(), builtins::add);
  }

  pub fn eval(&mut self, expr: String) -> String {
    let tokens = Parser::new().tokenize(expr);

    let func_name = &tokens[0];
    let args: Vec<String> = tokens[1..].into();

    match self.root_scope.resolve(func_name) {
      Some(func) => {
        func(&mut self.root_scope, args)
      }
      None => {
        panic!("Function not found!")
      }
    }
  }
}