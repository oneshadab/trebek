use std::{
    fs,
    io::{self, Write},
};
#[allow(dead_code)]
pub enum OutputStream {
    Stdout(io::Stdout),
    File(fs::File),
}

impl Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            OutputStream::Stdout(stdout) => stdout.write(buf),
            OutputStream::File(file) => file.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            OutputStream::Stdout(stdout) => stdout.flush(),
            OutputStream::File(file) => file.flush(),
        }
    }
}
