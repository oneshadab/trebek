use super::{types::Function, builtins, parser::Parser, scope::Scope, types::Expression, types::{Record, Symbol}};
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
    self.root_scope.set(String::from("+"), Record::Function(builtins::add));
    self.root_scope.set(String::from("def"), Record::Function(builtins::def));
    self.root_scope.set(String::from("print"), Record::Function(builtins::print));
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
      Record::Function(func) => { panic!("Function eval not supported!")}
      Record::Expression(expr) => { self.eval_expression(expr) }
      Record::Symbol(symbol) => { self.eval_symbol(symbol) }
      Record::Empty => {Record::Empty}
    }
  }

  fn eval_expression(&mut self, expr: &Expression) -> Record {
    let mut parser = Parser::new();
    let tokens = parser.tokenize(&parser.trim(&expr));

    let args: Vec<Record> = tokens[1..].into();

    match self.eval(&tokens[0]) {
      Record::Function(func) => {
        func(self, args)
      }
      other => {
        panic!("{:?} is not a function", other)
      }
    }
  }

  fn eval_symbol(&mut self, symbol: &Symbol) -> Record {
    self.root_scope.resolve(&symbol)
  }
}