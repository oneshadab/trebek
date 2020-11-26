pub struct Parser {}

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn tokenize(&self, expr: &String) -> Vec<String> {
    let mut tokens = Vec::new();

    let mut flush = |buffer: &mut Vec<char>| {
      if buffer.is_empty() {
        return ;
      }

      let token= buffer.clone().iter().collect();
      tokens.push(token);
      buffer.clear();
    };

    let mut buffer = Vec::new();
    let mut depth = 0;

    for ch in expr.chars() {
      if ch == '(' {
        depth += 1;
      }

      if ch == ')' {
        depth -= 1;
      }

      if depth == 0 && self.is_white_space(ch) {
        flush(&mut buffer);
      }
      else {
        buffer.push(ch);
      }
    }

    flush(&mut buffer);

    return tokens;
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
}