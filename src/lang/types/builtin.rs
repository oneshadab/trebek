use crate::lang::runner::Runner;

use super::record::Record;

pub type Builtin = fn(&mut Runner, &[Record]) -> Record;
