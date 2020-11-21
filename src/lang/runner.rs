use super::scope::Scope;
pub struct Runner {
  rootScope: Scope
}

impl Runner {
  pub fn new(&mut self) {
    self.rootScope = Scope::new();
  }
}