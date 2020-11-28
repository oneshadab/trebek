
#![cfg(test)]
extern crate test_generator;

pub mod tests {
  use trebek::lang::runtime::Runtime;
  use test_generator::test_resources;


  #[test_resources("tests/res/programs/*")]
  fn verify(program_dir: &str) {
    println!("[DBG] {}", program_dir);
    assert!(std::path::Path::new(program_dir).exists());
  }
}