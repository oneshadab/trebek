use super::types::Record;

pub struct Parser {
}

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn tokenize(&mut self, text: &String) -> Vec<Record> {
    let mut records = Vec::new();

    let mut buffer: Vec<char> = Vec::new();
    let mut depth = 0;

    for ch in text.chars() {
      if ch == '(' {
        depth += 1;
      }

      if ch == ')' {
        depth -= 1;
      }

      if depth == 0 && self.is_white_space(ch) {
        if !buffer.is_empty() {
          records.push( self.to_record(&buffer) );
          buffer.clear();
        }
      }
      else {
        buffer.push(ch);
      }
    }

    if !buffer.is_empty() {
      records.push( self.to_record(&buffer) );
      buffer.clear();
    }

    return records;
  }

  fn is_white_space(&self, ch: char) -> bool {
    let whitespace_chars = [' ', '\t', '\n'];
    return whitespace_chars.contains(&ch);
  }

  pub fn trim(&self, expr: &String) -> String {
    let char_buffer: Vec<_> = expr.chars().collect();

    let mut new_start = 0;
    while self.is_white_space(char_buffer[new_start]) || char_buffer[new_start] == '(' {
      new_start += 1;
    }

    let mut new_end = char_buffer.len() - 1;
    while self.is_white_space(char_buffer[new_end]) || char_buffer[new_end] == ')' {
      new_end -= 1;
    }

    let trimmed_buffer = char_buffer[new_start .. new_end+1].to_vec();
    return trimmed_buffer.into_iter().collect();
  }

  fn to_record(&mut self, buffer: &Vec<char>) -> Record {
    let token: String = buffer.iter().collect();

    match buffer[..] {
      [] => {
        Record::Empty
      }
      ['(', .., ')'] => {
        Record::Expression(token)
      },
      _ => {
        Record::Symbol(token)
      }
    }
  }
}