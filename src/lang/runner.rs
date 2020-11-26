use std::{rc::Rc, cell::RefCell};

use super::{builtins, parser::Parser, scope::Scope, types::expression::Expression, types::{record::Record, symbol::Symbol}};
pub struct Runner  {
  pub root_scope: Rc<RefCell<Scope>>,
  pub current_scope: Rc<RefCell<Scope>>
}

impl Runner {
  pub fn new() -> Runner {
    let scope = Rc::new(RefCell::new(Scope::new(None)));

    let mut runner = Runner {
      root_scope: scope.clone(),
      current_scope: scope.clone()
    };

    runner.init_builtins();

    runner
  }

  fn init_builtins(&mut self) {
    for builtin in builtins::get_builtins() {
      self.set_global(builtin.name.into(), Record::Builtin(builtin));
    }
  }

  pub fn run(&mut self, program: String) -> Record {
    let exprs = Parser::new().tokenize(&program);

    let mut out = Record::Empty;
    for expr in exprs {
      match expr {
        Record::Expression(expr) => {
          println!("[DBG] Executing expression: '{}'", expr);
          out = self.eval(&Record::Expression(expr));
        }
        r => {
          panic!("{:?} is not an expression", r)
        }
      }
    }
    return out;
  }

  pub fn set_global(&mut self, key: Symbol, val: Record) {
    let scope = &self.root_scope;
    scope.borrow_mut().set(key, val);
  }

  pub fn set_local(&mut self, key: Symbol, val: Record) {
    // To-do: Introduce local scope
    self.set_global(key, val)
  }

  pub fn eval(&mut self, record: &Record) -> Record {
    match record {
      Record::Expression(expr) => { self.eval_expression(expr) }
      Record::Symbol(symbol) => { self.eval_symbol(symbol) }
      Record::Empty => {Record::Empty}
      other => { panic!("{:?} evaluation is not supported", other)}
    }
  }

  fn eval_expression(&mut self, expr: &Expression) -> Record {
    let mut parser = Parser::new();
    let records = parser.tokenize_expression(expr);

    let func_record = &records[0];
    let arg_records = &records[1..];

    match self.eval(func_record) {
      Record::Builtin(builtin) => {
        builtin.apply(self, arg_records.into())
      }
      Record::Function(func) => {
        func.apply(self, arg_records.into())
      }
      other => { panic!("{:?} is not a function", other) }
    }
  }

  fn eval_symbol(&mut self, symbol: &Symbol) -> Record {
    let scope = &self.root_scope;
    let record = scope.borrow_mut().resolve(&symbol);
    match record {
      Record::Expression(expr) => { self.eval(&Record::Expression(expr))}
      other => { other }
    }
  }
}