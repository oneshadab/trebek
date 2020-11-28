use crate::lang::{types::function::Function, parser::Parser, runtime::Runtime, types::{builtin::Builtin, record::Record}};



pub fn get_builtins() -> Vec<Builtin>{
  vec![
    Builtin::new("fn", new_function),
  ]
}


pub fn new_function(_ctx: &mut Runtime, args: &[Record]) -> Record {
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
