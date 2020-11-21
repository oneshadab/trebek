use super::{types::Record, scope::Scope};

pub fn add(scope: &mut Scope, args: Vec<String>) -> Record {
  match &args[..] {
    [a, b] => {
      let i_a: i32 = a.parse().unwrap();
      let i_b: i32 = b.parse().unwrap();
      let i_result = i_a + i_b;

      return Record::Symbol(i_result.to_string());
    }
    _ => {
      panic!("Function called with incorrect number of args")
    }
  }
}

pub fn def(scope: &mut Scope, args: Vec<String>) -> Record {
  match &args[..] {
    [symbol, val] => {
      let entry = Record::Symbol(val.clone());
      scope.set(symbol.clone(), entry);

      return Record::Empty;
    }
    _ => {
      panic!("Function called with incorrect number of args")
    }
  }
}

pub fn print(scope: &mut Scope, args: Vec<String>) -> Record {
  match &args[..] {
    [symbol] => {
      println!("{}", symbol);
      Record::Empty
    }
    _ => {
      panic!("Function called with incorrect number of args")
    }
  }
}