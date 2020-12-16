

use std::{io::{self, BufReader, BufWriter}};

use super::{
  builtins,
  parser::Parser,
  scope::Scope,
  types::callable::Callable,
  types::expression::Expression,
  io_helpers::input_stream::InputStream,
  io_helpers::output_stream::OutputStream,
  types::{tobject::TObject, symbol::Symbol}
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
      self.set_global(builtin.name.into(), TObject::Builtin(builtin));
    }
  }

  pub fn run(&mut self, program: String) -> String {
    let exprs = Parser::new().tokenize(&program);

    let mut out = TObject::Empty;
    for expr in exprs {
      match expr {
        TObject::Expression(expr) => {
          out = self.eval(&TObject::Expression(expr));
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

  pub fn set_global(&mut self, key: Symbol, val: TObject) {
    self.root_scope().set(key, val);
  }

  pub fn set_local(&mut self, key: Symbol, val: TObject) {
    self.current_scope().set(key, val);
  }

  pub fn eval(&mut self, obj: &TObject) -> TObject {
    let output = match obj {
      TObject::Expression(expr) => { self.eval_expression(expr) }
      TObject::Symbol(symbol) => { self.eval_symbol(symbol) }
      TObject::Empty => {TObject::Empty}
      other => { panic!("{:?} evaluation is not supported", other)}
    };

    eprintln!("[DBG] Executing '{:?}' || Output: '{:?}'", obj, output);

    output
  }

  fn eval_expression(&mut self, expr: &Expression) -> TObject {
    let mut parser = Parser::new();
    let objs = parser.tokenize_expression(expr);

    let func_obj = &objs[0];
    let arg_objs = &objs[1..];


    let callable: Box<dyn Callable> = match self.eval(func_obj) {
      TObject::Builtin(builtin) => { Box::new(builtin) }
      TObject::Closure(func) => { Box::new(func) }
      other => { panic!("{:?} is not callable", other) }
    };


    let parent_scope = self.current_scope_id;
    self.new_child_scope();

    let output = callable.call(self, arg_objs.into());

    self.restore_scope(parent_scope);

    output
  }

  fn eval_symbol(&mut self, symbol: &Symbol) -> TObject {
    let default = TObject::Symbol(symbol.clone().into());
    let obj = self.recursive_lookup(symbol).unwrap_or(default);

    match obj {
      TObject::Expression(expr) => { self.eval(&TObject::Expression(expr))}
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


  fn recursive_lookup(&mut self, symbol: &Symbol) -> Option<TObject> {
    for scope in self.scope_chain() {
      if let Some(obj) = scope.lookup(symbol) {
        return Some(obj);
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
