use crate::lang::{types::closure::Closure, parser::Parser, runtime::Runtime, types::{builtin::Builtin, tobject::TObject}};



pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("fn", create_function),
    Builtin::new("defn", define_function),
  ]
}

fn create_function(ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
    match &args[..] {
    [
      TObject::Expression(params_expr),
      TObject::Expression(body_expr)
    ] => {
      let func = init_function(ctx, params_expr, body_expr);
      TObject::Closure(func)
    }
    _ => {
      panic!("'print' called with incorrect number of args")
    }
  }
}

fn define_function(ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
  match &args[..] {
    [
      TObject::Symbol(symbol),
      TObject::Expression(params_expr),
      TObject::Expression(body_expr),
    ] => {
      let func = init_function(ctx, params_expr, body_expr);
      ctx.set_global(symbol.clone(), TObject::Closure(func));

      TObject::Empty
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
        TObject::Symbol(s) => { s },
        other => { panic!("{:?} is not a proper param!", other) }
      }
    })
    .collect();

  Closure::new(ctx, qualified_params, body.into())
}
