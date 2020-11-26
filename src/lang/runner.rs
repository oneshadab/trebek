use super::{builtins, parser::Parser, scope::Scope, types::Expression, types::{Record, Symbol}};
pub struct Runner {
  pub root_scope: Scope,
  pub current_scope: Scope
}

impl Runner {
  pub fn new() -> Runner{
    let mut runner = Runner {
      root_scope: Scope::new(),
      current_scope: Scope::new()
    };

    runner.init_builtins();

    return runner;
  }

  fn init_builtins(&mut self) {
    self.root_scope.set(String::from("+"), Record::Builtin(builtins::add));
    self.root_scope.set(String::from("def"), Record::Builtin(builtins::def));
    self.root_scope.set(String::from("print"), Record::Builtin(builtins::print));
  }

  pub fn run(&mut self, program: String) -> Record {
    let exprs = Parser::new().tokenize(&program);

    let mut out = Record::Empty;
    for expr in exprs {
      match expr {
        Record::Expression(expr) => {
          println!("[Executing expression]: '{}'", expr);
          out = self.eval(&Record::Expression(expr));
        }
        r => {
          panic!("{:?} is not an expression", r)
        }
      }
    }
    return out;
  }

  pub fn eval(&mut self, record: &Record) -> Record {
    match record {
      Record::Builtin(_) => { panic!("Function eval not supported!")}
      Record::Expression(expr) => { self.eval_expression(expr) }
      Record::Symbol(symbol) => { self.eval_symbol(symbol) }
      Record::Empty => {Record::Empty}
    }
  }

  fn eval_expression(&mut self, expr: &Expression) -> Record {
    let mut parser = Parser::new();
    let records = parser.tokenize_expression(expr);

    let func_record = &records[0];
    let arg_records = &records[1..];

    let func = match self.eval(func_record) {
      Record::Builtin(func) => { func }
      other => { panic!("{:?} is not a function", other) }
    };

    func(self, arg_records)
  }

  fn eval_symbol(&mut self, symbol: &Symbol) -> Record {
    self.root_scope.resolve(&symbol)
  }

}