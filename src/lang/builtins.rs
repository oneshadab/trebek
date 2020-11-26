use super::{types::Record, scope::Scope};

pub fn add(scope: &mut Scope, args: Vec<String>) -> Record {
  match &args[..] {
    [arg, other_arg] => {
      let val = scope.resolve(arg);
      let other_val = scope.resolve(other_arg);

      match (val, other_val) {
        (Record::Symbol(a), Record::Symbol(b)) => {
          let i_a: i32 = a.parse().unwrap();
          let i_b: i32 = b.parse().unwrap();
          let i_result = i_a + i_b;

          return Record::Symbol(i_result.to_string());
        }
        _ => {
          panic!("'add' called with incorrect params")
        }
      }
    }
    _ => {
      panic!("'add' called with incorrect number of args")
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
      panic!("'def' called with incorrect number of args")
    }
  }
}

pub fn print(scope: &mut Scope, args: Vec<String>) -> Record {
  match &args[..] {
    [symbol] => {
      println!("{:?}", scope.resolve(symbol));
      Record::Empty
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}