use super::types::builtin::Builtin;

mod conditional;
mod function;
mod io;
mod math;
mod scope;

pub fn get_builtins() -> Vec<Builtin> {
    [
        math::get_builtins(),
        io::get_builtins(),
        function::get_builtins(),
        conditional::get_builtins(),
        scope::get_builtins(),
    ]
    .concat()
}
