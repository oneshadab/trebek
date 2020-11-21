use super::{parser::Parser, scope::Scope};
pub struct Runner {
  rootScope: Scope
}

impl Runner {
  pub fn new() -> Runner{
    let mut defaultScope = Scope::new();

    defaultScope.set(
      String::from('+'),
      |args: Vec<String>| -> String {
        match &args[..] {
          [a, b] => {
            let i_a: i32 = a.parse().unwrap();
            let i_b: i32 = b.parse().unwrap();

            return (i_a + i_b).to_string();
          }
          _ => {
            panic!("Missing args for function call")
          }
        }
      }
    );

    Runner {
      rootScope: defaultScope
    }
  }

  pub fn eval(&mut self, expr: String) -> String {
    let mut tokens = Parser::new().tokenize(expr);

    let funcName = &tokens[0];
    let args: Vec<String> = tokens[1..].into();

    match self.rootScope.resolve(funcName).unwrap() {
      func => {
        func(args)
      }
      _ => {
        panic!("Function not found!")
      }
    }
  }
}