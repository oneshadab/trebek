use std::{
    env,
    io::{self, BufReader, BufWriter},
};

use crate::misc::RuntimeResult;

use super::{
    builtins,
    io_helpers::input_stream::InputStream,
    io_helpers::output_stream::OutputStream,
    memory::{
        gc::GarbageCollector,
        object_heap::{ObjectHeap, ObjectId},
    },
    types::callable::Callable,
    types::list::List,
    types::{scope::Scope, t_object::TObject},
};

pub struct Runtime {
    heap: ObjectHeap,
    collector: GarbageCollector,

    pub root_scope_id: ObjectId,
    pub current_scope_id: ObjectId,

    pub reader: BufReader<InputStream>,
    pub writer: BufWriter<OutputStream>,
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
            writer: stdout_writer,
        };

        runtime.init_builtins();

        runtime
    }

    fn init_builtins(&mut self) {
        for builtin in builtins::get_builtins() {
            self.set_global(builtin.name.into(), TObject::Builtin(builtin));
        }
    }

    pub fn set_global(&mut self, symbol: String, obj: TObject) {
        self.add_to_scope(self.root_scope_id, symbol, obj)
    }

    pub fn set_local(&mut self, symbol: String, obj: TObject) {
        self.add_to_scope(self.current_scope_id, symbol, obj)
    }

    pub fn add_to_scope(&mut self, scope_id: ObjectId, symbol: String, obj: TObject) {
        let key = symbol;
        let val = self.heap.add(obj);

        self.get_mut_scope(scope_id).set(key, val);
    }

    pub fn eval(&mut self, obj: &TObject) -> RuntimeResult<TObject> {
        self.save_current_scope();

        self.new_child_scope();

        let output = match obj {
            TObject::List(expr) => self.eval_expression(expr)?,
            TObject::Symbol(symbol) => self.eval_symbol(symbol)?,
            TObject::String(s) => TObject::String(s.clone()),
            TObject::Empty => TObject::Empty,
            other => Err(format!("`{}` evaluation is not supported", other))?,
        };

        if let Ok(_) = env::var("DEBUG") {
            eprintln!("[DBG] Executing '{:?}' || Output: '{:?}'", obj, output);
        }

        self.restore_saved_scope();

        let obj_id = self.heap.add(output.clone());
        let current_scope = self.get_mut_scope(self.current_scope_id);
        current_scope.push(obj_id);

        self.run_gc();

        Ok(output)
    }

    fn eval_expression(&mut self, list: &List) -> RuntimeResult<TObject> {
        let func_obj = &list[0];
        let arg_objs = &list[1..];

        let callable: Box<dyn Callable> = match self.eval(func_obj)? {
            TObject::Builtin(builtin) => Box::new(builtin),
            TObject::Closure(func) => Box::new(func),
            other => Err(format!("{:?} is not callable", other))?,
        };

        callable.call(self, arg_objs.into())
    }

    fn eval_symbol(&mut self, symbol: &String) -> RuntimeResult<TObject> {
        // Lookup in scope first
        if let Some(v) = self.lookup(symbol) {
            return Ok(v.clone());
        }

        // Allow numbers to remain as symbols
        if symbol.parse::<i64>().is_ok() {
            return Ok(TObject::Symbol(symbol.clone()));
        }

        Err(format!("Symbol `{}` does not exist in scope", symbol))
    }

    pub fn new_child_scope(&mut self) {
        let new_scope = Scope::new(Some(self.current_scope_id));
        self.current_scope_id = self.heap.add(TObject::Scope(new_scope));
    }

    pub fn save_current_scope(&mut self) {
        let current_scope_id = self.current_scope_id;
        let root_scope = self.get_mut_scope(self.root_scope_id);

        root_scope.push(current_scope_id);
    }

    pub fn restore_saved_scope(&mut self) {
        let root_scope = self.get_mut_scope(self.root_scope_id);
        let saved_scope_id = root_scope.pop();

        self.set_scope(saved_scope_id);
    }

    pub fn set_scope(&mut self, scope_id: usize) {
        self.current_scope_id = scope_id;
    }

    pub fn lookup(&self, symbol: &String) -> Option<&TObject> {
        for scope in self.scope_chain() {
            if let Some(obj_id) = scope.lookup(symbol) {
                return self.heap.get(obj_id);
            }
        }
        None
    }

    fn get_scope(&self, scope_id: ObjectId) -> &Scope {
        match self.heap.get(scope_id) {
            Some(TObject::Scope(scope)) => scope,
            _ => {
                panic!("Scope not found for id {}", scope_id)
            }
        }
    }

    fn get_mut_scope(&mut self, scope_id: ObjectId) -> &mut Scope {
        match self.heap.get_mut(scope_id) {
            Some(TObject::Scope(scope)) => scope,
            _ => {
                panic!("Scope not found for id {}", scope_id)
            }
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
        }

        chain
    }

    fn run_gc(&mut self) {
        self.collector
            .collect(self.current_scope_id, &mut self.heap);
    }
}
