use super::scope::Scope;

pub type Function = fn(&mut Scope, Vec<String>) -> String;
