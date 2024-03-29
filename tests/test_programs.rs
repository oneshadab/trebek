#![cfg(test)]
extern crate test_generator;

pub mod tests {
    use std::{
        fs::{self, read_to_string},
        io,
        io::Write,
        path::Path,
    };

    use tempfile;
    use test_generator::test_resources;

    use trebek::{
        io_helpers::{input_stream::InputStream, output_stream::OutputStream},
        repl::Repl,
    };

    #[test_resources("tests/res/programs/**")]
    fn verify(program_dir: &str) {
        let dir = Path::new(program_dir);

        let program_path = dir.join("program.tr");

        let program = read_to_string(program_path);
        if program.is_err() {
            return;
        }

        let program = program.expect("Program not found!");

        let expected_output_path = dir.join("expected_output.txt");
        let expected_output = read_to_string(expected_output_path).unwrap_or("".into());

        let input_file_path = dir.join("input.txt");
        let input_file = fs::File::open(input_file_path).unwrap_or(tempfile::tempfile().unwrap());

        let output_file = tempfile::NamedTempFile::new().unwrap();

        let mut runner = Repl::new();
        runner.runtime.reader = io::BufReader::new(InputStream::File(input_file));
        runner.runtime.writer =
            io::BufWriter::new(OutputStream::File(output_file.reopen().unwrap()));

        runner.eval(program).unwrap();

        runner.runtime.writer.flush().unwrap();

        let output = read_to_string(output_file).unwrap();
        assert_eq!(output, expected_output);
    }
}
