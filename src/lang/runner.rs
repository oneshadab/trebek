use super::{parser::Parser, scope::Scope};
pub struct Runner {
  root_scope: Scope
}

impl Runner {
  pub fn new() -> Runner{
    let mut default_scope = Scope::new();

    default_scope.set(
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
      root_scope: default_scope
    }
  }

  pub fn eval(&mut self, expr: String) -> String {
    let tokens = Parser::new().tokenize(expr);

    let func_name = &tokens[0];
    let args: Vec<String> = tokens[1..].into();

    match self.root_scope.resolve(func_name) {
      Some(func) => {
        func(args)
      }
      None => {
        panic!("Function not found!")
      }
    }
  }
}