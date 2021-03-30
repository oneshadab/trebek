use crate::runtime::{Runtime, RuntimeResult};

use super::t_object::TObject;

pub trait Callable {
    fn call(&self, ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject>;
}
