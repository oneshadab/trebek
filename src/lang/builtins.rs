use super::scope::Scope;

pub fn add(scope: &mut Scope, args: Vec<String>) -> String {
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