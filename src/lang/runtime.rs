

use std::{fs::{self, File}, io::{self, BufReader, BufWriter, Read, Write}};

use super::{
  builtins,
  parser::Parser,
  scope::Scope,
  types::callable::Callable,
  types::expression::Expression,
  io_helpers::input_stream::InputStream,
  io_helpers::output_stream::OutputStream,
  types::{record::Record, symbol::Symbol}
};

pub struct Runtime  {
  scopes: Vec<Scope>,

  pub root_scope_id: usize,
  pub current_scope_id: usize,

  pub reader: BufReader<InputStream>,
  pub writer: BufWriter<OutputStream>
}

impl Runtime {
  pub fn new() -> Runtime {
    let scopes = vec![Scope::new(None)];

    let stdin_reader = BufReader::new(InputStream::Stdin(io::stdin()));
    let stdout_writer = BufWriter::new(OutputStream::Stdout(io::stdout()));


    let mut runtime = Runtime {
      scopes,

      root_scope_id: 0,
      current_scope_id: 0,

      reader: stdin_reader,
      writer: stdout_writer
    };

    runtime.init_builtins();

    runtime
  }

  fn init_builtins(&mut self) {
    for builtin in builtins::get_builtins() {
      self.set_global(builtin.name.into(), Record::Builtin(builtin));
    }
  }

  pub fn run(&mut self, program: String) -> String {
    let exprs = Parser::new().tokenize(&program);

    let mut out = Record::Empty;
    for expr in exprs {
      match expr {
        Record::Expression(expr) => {
          eprintln!("[DBG] Executing expression: '{}'", expr);
          out = self.eval(&Record::Expression(expr));
        }
        r => {
          panic!("{:?} is not an expression", r)
        }
      }
    }

    return format!("{:?}", out);
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


    let callable: Box<Callable> = match self.eval(func_record) {
      Record::Builtin(builtin) => { Box::new(builtin) }
      Record::Function(func) => { Box::new(func) }
      other => { panic!("{:?} is not a function", other) }
    };


    let scope_before_call = self.current_scope_id;

    let output = callable.call(self, arg_records.into());

    // Always restore the scope in-case it was changed by callee
    self.restore_scope(scope_before_call);

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

  pub fn new_child_scope(&mut self) {
    self.scopes.push(Scope::new(Some(self.current_scope_id)));
    self.current_scope_id = self.scopes.len() - 1;
  }

  pub fn restore_scope(&mut self, scope_id: usize) {
    self.current_scope_id = scope_id;
  }


  fn recursive_lookup(&mut self, symbol: &Symbol) -> Option<Record> {
    for scope in self.scope_chain() {
      if let Some(record) = scope.lookup(symbol) {
        return Some(record);
      }
    }
    None
  }

  fn scope_chain(&self) -> Vec<&Scope> {
    let mut chain = Vec::new();
    let mut scope = &self.scopes[self.current_scope_id];

    loop {
      chain.push(scope);

      if let None = scope.parent_scope_id {
        break;
      }
      scope = &self.scopes[scope.parent_scope_id.unwrap()];
    };

    chain
  }
}
