use std::{fs, io::{self, Read}};
#[allow(dead_code)]
pub enum InputStream {
  Stdin(io::Stdin),
  File(fs::File)
}

impl Read for InputStream {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>  {
    match self {
        InputStream::Stdin(stdin) => { stdin.read(buf) }
        InputStream::File(file) => { file.read(buf) }
    }
  }
}
