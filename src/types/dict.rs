use std::collections::HashMap;

use crate::{misc::RuntimeResult, runtime::Runtime};

use super::{callable::Callable, t_object::TObject};

pub type Dict = HashMap<String, TObject>;

impl Callable for Dict {
    fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
        if args.len() != 1 {
            return Err(format!("'dict-get' called with incorrect number of args"));
        }

        let obj = &args[0];
        let key = ctx.eval(obj)?;

        match key {
            TObject::Symbol(key) => {
                let val = self.get(&key).unwrap_or(&TObject::Empty);
                Ok(val.clone())
            },
            _ => Err(format!("'dict-get' called with incorrect number of args")),
        }
    }
}
