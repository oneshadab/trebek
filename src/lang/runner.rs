use super::{parser::Parser, scope::Scope};
pub struct Runner {
  rootScope: Scope
}

impl Runner {
  pub fn new(&mut self) {
    self.rootScope = Scope::new();

    self.rootScope.set(
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
  }

}