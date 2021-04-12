use crate::{misc::RuntimeResult, runtime::Runtime};
use std::fmt;

use super::{callable::Callable, t_object::TObject};

type Func = fn(&mut Runtime, Vec<TObject>) -> RuntimeResult<TObject>;

#[derive(Clone)]
pub struct Builtin {
    pub name: &'static str,
    pub func: Func,
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Builtin {
    pub fn new(name: &'static str, func: Func) -> Builtin {
        Builtin { name, func }
    }
}

impl Callable for Builtin {
    fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
        (self.func)(ctx, args)
    }
}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.name[..]).finish()
    }
}
