#![cfg(test)]
extern crate test_generator;

pub mod tests {
  use std::{path::Path, fs::read_to_string};

  use test_generator::test_resources;

  use trebek::lang::runtime::Runtime;


  #[test_resources("tests/res/programs/*")]
  fn verify(program_dir: &str) {
    let dir = Path::new(program_dir);

    let program_path = dir.join("program.tr");
    let program = read_to_string(program_path).expect("Program not found!");

    let expected_output_path = dir.join("expected_output.txt");
    let expected_output = read_to_string(expected_output_path).expect("Expected output not found!");


    let mut runtime = Runtime::new();
    let output = runtime.run(program);


    assert_eq!(output, expected_output);
  }
}