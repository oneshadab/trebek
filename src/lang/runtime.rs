use std::{rc::Rc, cell::RefCell};

use super::{builtins, parser::Parser, scope::Scope, types::expression::Expression, types::{record::Record, symbol::Symbol}};
pub struct Runtime  {
  scopes: Vec<Scope>,

  pub root_scope_id: usize,
  pub current_scope_id: usize
}

impl Runtime {
  pub fn new() -> Runtime {
    let scopes = vec![Scope::new(None)];

    let mut runtime = Runtime {
      scopes,
      root_scope_id: 0,
      current_scope_id: 0,
    };

    runtime.init_builtins();

    runtime
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

  pub fn root_scope(&mut self) -> &mut Scope{
    &mut self.scopes[self.root_scope_id]
  }

  pub fn current_scope(&mut self) -> &mut Scope{
    &mut self.scopes[self.current_scope_id]
  }

  pub fn set_global(&mut self, key: Symbol, val: Record) {
    self.root_scope().set(key, val);
  }

  pub fn set_local(&mut self, key: Symbol, val: Record) {
    self.current_scope().set(key, val);
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

    self.push_scope();

    let output = match self.eval(func_record) {
      Record::Builtin(builtin) => { builtin.apply(self, arg_records.into()) }
      Record::Function(func) => { func.apply(self, arg_records.into()) }
      other => { panic!("{:?} is not a function", other) }
    };

    self.pop_scope();

    output
  }

  fn eval_symbol(&mut self, symbol: &Symbol) -> Record {
    let default = Record::Symbol(symbol.clone().into());
    let record = self.recursive_lookup(symbol).unwrap_or(default);

    match record {
      Record::Expression(expr) => { self.eval(&Record::Expression(expr))}
      other => { other }
    }
  }

  fn push_scope(&mut self) {
    self.scopes.push(Scope::new(Some(self.current_scope_id)));
    self.current_scope_id = self.scopes.len() - 1;
  }

  fn pop_scope(&mut self) {
    self.current_scope_id = self.current_scope().parent_scope_id.unwrap();
  }

  fn recursive_lookup(&mut self, symbol: &Symbol) -> Option<Record> {
    let mut scope = &self.scopes[self.current_scope_id];
    loop {
      if let Some(record) = scope.lookup(symbol) {
        return Some(record);
      }

      if let None = scope.parent_scope_id {
        return None;
      }

      scope = &self.scopes[scope.parent_scope_id.unwrap()];
    }
  }
}