use super::types::builtin::Builtin;

mod conditional;
mod function;
mod internal_utils;
mod io;
mod math;
mod scope;
mod string;
mod list;

pub fn get_builtins() -> Vec<Builtin> {
    [
        math::get_builtins(),
        io::get_builtins(),
        function::get_builtins(),
        conditional::get_builtins(),
        scope::get_builtins(),
        string::get_builtins(),
        list::get_builtins(),
    ]
    .concat()
}
