#![cfg(test)]
extern crate test_generator;

pub mod tests {
  use std::{fs::{self, read_to_string}, io, io::BufRead, path::Path, io::Write};

  use test_generator::test_resources;
  use tempfile;

  use trebek::lang::{io_helpers::{output_stream::OutputStream}, runtime::Runtime};


  #[test_resources("tests/res/programs/*")]
  fn verify(program_dir: &str) {
    let dir = Path::new(program_dir);

    let program_path = dir.join("program.tr");
    let program = read_to_string(program_path).expect("Program not found!");

    let expected_output_path = dir.join("expected_output.txt");
    let expected_output = read_to_string(expected_output_path).expect("Expected output not found!");


    let output_file = tempfile::NamedTempFile::new().unwrap();
    let writeable_output_file = output_file.reopen().unwrap();

    let mut runtime = Runtime::new();
    runtime.writer =  io::BufWriter::new(OutputStream::File(writeable_output_file));

    runtime.run(program);

    runtime.writer.flush().unwrap();

    let output = read_to_string(output_file).unwrap();
    assert_eq!(output, expected_output);
  }
}
