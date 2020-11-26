use super::{types::function::Function, parser::Parser, runner::Runner, types::{builtin::Builtin, record::Record}};

pub fn get_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new("+", add),
    Builtin::new("def", def),
    Builtin::new("print", print),
    Builtin::new("fn", new_function),
  ]
}

pub fn add(ctx: &mut Runner, args: &[Record]) -> Record {
  match args {
    [arg, other_arg] => {
      let val = ctx.eval(arg);
      let other_val = ctx.eval(other_arg);

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

pub fn def(ctx: &mut Runner, args: &[Record]) -> Record {
  match args {
    [Record::Symbol(symbol), val] => {
      ctx.set_global(symbol.clone(), val.clone());

      return Record::Empty;
    }
    _ => {
      panic!("'def' called with incorrect number of args")
    }
  }
}

pub fn print(ctx: &mut Runner, args: &[Record]) -> Record {
  match args {
    [symbol] => {
      let val = ctx.eval(symbol);
      println!("{:?}", val);
      Record::Empty
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}

pub fn new_function(ctx: &mut Runner, args: &[Record]) -> Record {
  match args {
    [Record::Expression(params_expr), Record::Expression(body)] => {
      let mut parser = Parser::new();
      let params = parser.tokenize_expression(&params_expr);

      let qualified_params = params
        .into_iter()
        .map(|p| {
          match p {
            Record::Symbol(s) => { s },
            other => { panic!("{:?} is not a proper param!", other) }
          }
        })
        .collect();

      let func = Function::new(qualified_params, body.into());
      Record::Function(func)
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}
