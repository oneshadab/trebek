use super::types::builtin::Builtin;

mod math;
mod io;
mod function;
mod control_flow;
mod scope;

pub fn get_builtins() -> Vec<Builtin> {
  [
    math::get_builtins(),
    io::get_builtins(),
    function::get_builtins(),
    control_flow::get_builtins(),
    scope::get_builtins(),
  ].concat()
}

//
// Builtin::new("def", def),
// Builtin::new("print", print),
// Builtin::new("fn", new_function),
// Builtin::new("if", cond_if),
// Builtin::new("=", is_equal),