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
    self.root_scope.set(String::from("+"), Record::Function(builtins::add));
    self.root_scope.set(String::from("def"), Record::Function(builtins::def));
    self.root_scope.set(String::from("print"), Record::Function(builtins::print));
  }

  pub fn run(&mut self, program: String) -> Record {
    let exprs = Parser::new().tokenize(&program);

    let mut out = Record::Empty;
    for expr in exprs {
      println!("[Executing expression]: '{}'", expr);
      out = self.eval(Record::Expression(expr));
    }
    return out;
  }

  pub fn eval(&mut self, record: Record) -> Record {
    match record {
      Record::Function(func) => { panic!("Function eval not supported!")}
      Record::Expression(expr) => { self.eval_expression(expr) }
      Record::Symbol(symbol) => { self.eval_symbol(symbol) }
      Record::Empty => {Record::Empty}
    }
  }

  fn eval_expression(&mut self, expr: Expression) -> Record {
    let parser = Parser::new();
    let tokens = parser.tokenize(&parser.trim(&expr));

    let func_name = &tokens[0];
    let args: Vec<String> = tokens[1..].into();
    match self.root_scope.lookup(func_name) {
      Some(record) => {
        match record {
          Record::Function(func) => {
            func(self, args)
          },
          Record::Symbol(symbol) => {
            panic!("Symbol is not callable")
          },
          Record::Expression(expr) => {
            panic!("Expression calling isn't supported (yet)")
          },
          Record::Empty => {
            Record::Empty
          }
        }
      }
      None => {
        panic!("Function not found!")
      }
    }
  }

  fn eval_symbol(&mut self, symbol: Symbol) -> Record {
    self.root_scope.resolve(&symbol)
  }
}