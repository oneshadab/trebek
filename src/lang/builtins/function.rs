use crate::lang::{types::closure::Closure, parser::Parser, runtime::Runtime, types::{builtin::Builtin, record::Record}};



pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("fn", create_function),
    Builtin::new("defn", define_function),
  ]
}

fn create_function(ctx: &mut Runtime, args: Vec<Record>) -> Record {
    match &args[..] {
    [
      Record::Expression(params_expr),
      Record::Expression(body_expr)
    ] => {
      let func = init_function(ctx, params_expr, body_expr);
      Record::Closure(func)
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}

fn define_function(ctx: &mut Runtime, args: Vec<Record>) -> Record {
  match &args[..] {
    [
      Record::Symbol(symbol),
      Record::Expression(params_expr),
      Record::Expression(body_expr),
    ] => {
      let func = init_function(ctx, params_expr, body_expr);
      ctx.set_global(symbol.clone(), Record::Closure(func));

      Record::Empty
    }
    _ => {
      panic!("'defn' called with incorrect number of args")
    }
  }
}

fn init_function(ctx: &mut Runtime, params_expr: &String, body: &String) -> Closure {
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

  Closure::new(ctx, qualified_params, body.into())
}
