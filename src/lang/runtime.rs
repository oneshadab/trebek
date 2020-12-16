

use std::{io::{self, BufReader, BufWriter}};

use super::{builtins, io_helpers::input_stream::InputStream, io_helpers::output_stream::OutputStream, memory::{gc::GarbageCollector, object_heap::{ObjectHeap, ObjectId}}, types::callable::Callable, types::list::List, types::{scope::Scope, symbol::Symbol, t_object::TObject}};

pub struct Runtime  {
  heap: ObjectHeap,
  collector: GarbageCollector,

  pub root_scope_id: ObjectId,
  pub current_scope_id: ObjectId,

  pub reader: BufReader<InputStream>,
  pub writer: BufWriter<OutputStream>
}

impl Runtime {
  pub fn new() -> Runtime {
    let mut heap = ObjectHeap::new();

    let scope = Scope::new(None);
    let scope_id = heap.add(TObject::Scope(scope));

    let stdin_reader = BufReader::new(InputStream::Stdin(io::stdin()));
    let stdout_writer = BufWriter::new(OutputStream::Stdout(io::stdout()));

    let mut runtime = Runtime {
      heap,
      collector: GarbageCollector::new(),

      root_scope_id: scope_id,
      current_scope_id: scope_id,

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


  pub fn set_global(&mut self, key: Symbol, obj: TObject) {
    let obj_id = self.heap.add(obj);
    let root_scope = self.get_mut_scope(self.root_scope_id);

    root_scope.set(key, obj_id);
  }

  pub fn set_local(&mut self, key: Symbol, obj: TObject) {
    let obj_id = self.heap.add(obj);
    let current_scope = self.get_mut_scope(self.current_scope_id);

    current_scope.set(key, obj_id);
  }

  pub fn eval(&mut self, obj: &TObject) -> TObject {
    let output = match obj {
      TObject::List(expr) => { self.eval_expression(expr) }
      TObject::Symbol(symbol) => { self.eval_symbol(symbol) }
      TObject::Empty => {TObject::Empty}
      other => { panic!("{:?} evaluation is not supported", other)}
    };

    eprintln!("[DBG] Executing '{:?}' || Output: '{:?}'", obj, output);

    output
  }

  fn eval_expression(&mut self, list: &List) -> TObject {
    let func_obj = &list[0];
    let arg_objs = &list[1..];

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
    self.recursive_lookup(symbol).unwrap_or(&default).clone()
  }

  pub fn new_child_scope(&mut self) {
    let new_scope = Scope::new(Some(self.current_scope_id));
    self.current_scope_id = self.heap.add(TObject::Scope(new_scope));
  }

  pub fn restore_scope(&mut self, scope_id: usize) {
    self.current_scope_id = scope_id;
  }


  fn recursive_lookup(&mut self, symbol: &Symbol) -> Option<&TObject> {
    for scope in self.scope_chain() {
      if let Some(obj_id) = scope.lookup(symbol) {
        return self.heap.get(obj_id);
      }
    }
    None
  }

  fn get_scope(&self, scope_id: ObjectId) -> &Scope {
    match self.heap.get(scope_id) {
      Some(TObject::Scope(scope)) => { scope }
      _ => { panic!("Scope not found for id {}", scope_id) }
    }
  }

  fn get_mut_scope(&mut self, scope_id: ObjectId) -> &mut Scope {
    match self.heap.get_mut(scope_id) {
      Some(TObject::Scope(scope)) => { scope }
      _ => { panic!("Scope not found for id {}", scope_id) }
    }
  }

  fn scope_chain(&self) -> Vec<Scope> {
    let mut chain = Vec::new();
    let mut scope = self.get_scope(self.current_scope_id);

    loop {
      chain.push(scope.clone());

      if let None = scope.parent_scope_id {
        break;
      }

      scope = self.get_scope(scope.parent_scope_id.unwrap());
    };

    chain
  }

  fn run_gc(&mut self) {
    self.collector.collect(self.root_scope_id, &mut self.heap);
  }
}
