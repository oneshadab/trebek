#![cfg(test)]
extern crate test_generator;

pub mod tests {
  use std::{fs::{self, read_to_string}, io, path::Path, io::Write};

  use test_generator::test_resources;
  use tempfile;

  use trebek::lang::{io_helpers::{input_stream::InputStream, output_stream::OutputStream}, runner::Runner};


  #[test_resources("tests/res/programs/*")]
  fn verify(program_dir: &str) {
    let dir = Path::new(program_dir);

    let program_path = dir.join("program.tr");
    let program = read_to_string(program_path).expect("Program not found!");

    let expected_output_path = dir.join("expected_output.txt");
    let expected_output = read_to_string(expected_output_path).expect("Expected output not found!");

    let input_file_path = dir.join("input.txt");
    let input_file = fs::File::open(input_file_path).unwrap_or(tempfile::tempfile().unwrap());

    let output_file = tempfile::NamedTempFile::new().unwrap();

    let mut runner = Runner::new();
    runner.runtime.reader = io::BufReader::new(InputStream::File(input_file));
    runner.runtime.writer =  io::BufWriter::new(OutputStream::File(output_file.reopen().unwrap()));

    runner.run(program);

    runner.runtime.writer.flush().unwrap();

    let output = read_to_string(output_file).unwrap();
    assert_eq!(output, expected_output);
  }
}
