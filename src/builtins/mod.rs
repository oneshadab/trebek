use super::types::builtin::Builtin;

mod base;
mod conditional;
mod dict;
mod function;
mod io;
mod list;
mod math;
mod scope;
mod string;

pub fn get_builtins() -> Vec<Builtin> {
    [
        base::get_builtins(),
        conditional::get_builtins(),
        dict::get_builtins(),
        function::get_builtins(),
        io::get_builtins(),
        list::get_builtins(),
        math::get_builtins(),
        scope::get_builtins(),
        string::get_builtins(),
    ]
    .concat()
}
