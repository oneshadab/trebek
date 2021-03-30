macro_rules! create_builtin {
  ($name, $body) => {
    fn create_function(ctx: &mut Runtime, args: Vec<TObject>) -> TObject {
      match &args[..] {
          [TObject::List(params_expr), TObject::List(body_expr)] => {
              let func = init_function(ctx, params_expr, body_expr);
              TObject::Closure(func)
          }
          _ => {
              Err(format!("'print' called with incorrect number of args"))
          }
      }
    }
  }
}
