use std::collections::VecDeque;

use crate::types::symbol::Symbol;
use crate::types::t_object::TObject;

pub static TRUE: &str = "true";
pub static FALSE: &str = "false";

pub fn get_constants() -> Vec<(Symbol, TObject)> {
  return vec![
    ("true".into(), TObject::Symbol(TRUE.into())),
    ("false".into(), TObject::Symbol(FALSE.into())),
    ("nil".into(), TObject::List(VecDeque::new())),
  ]
}
