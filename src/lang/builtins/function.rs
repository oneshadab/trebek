use crate::lang::{types::function::Function, parser::Parser, runtime::Runtime, types::{builtin::Builtin, record::Record}};



pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("fn", create_function),
    Builtin::new("defn", define_function),
  ]
}

fn create_function(_ctx: &mut Runtime, args: &[Record]) -> Record {
  match args {
    [
      Record::Expression(params_expr),
      Record::Expression(body_expr)
    ] => {
      let func = init_function(params_expr, body_expr);
      Record::Function(func)
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}

fn define_function(ctx: &mut Runtime, args: &[Record]) -> Record {
  match args {
    [
      Record::Symbol(symbol),
      Record::Expression(params_expr),
      Record::Expression(body_expr),
    ] => {
      let func = init_function(params_expr, body_expr);
      ctx.set_global(symbol.clone(), Record::Function(func));

      Record::Empty
    }
    _ => {
      panic!("'defn' called with incorrect number of args")
    }
  }
}

fn init_function(params_expr: &String, body: &String) -> Function {
  let mut parser = Parser::new();
  let params = parser.tokenize_expression(params_expr);

  let qualified_params = params
    .into_iter()
    .map(|p| {
      match p {
        Record::Symbol(s) => { s },
        other => { panic!("{:?} is not a proper param!", other) }
      }
    })
    .collect();

  Function::new(qualified_params, body.into())
}