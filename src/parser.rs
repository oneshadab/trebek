use crate::runtime::RuntimeResult;

use super::types::{list::List, t_object::TObject};

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&mut self, expr: &String) -> RuntimeResult<List> {
        let tokens = self.tokenize(&self.trim_expression(expr)?);

        tokens
            .iter()
            .map(|token| -> RuntimeResult<TObject> {
                let chars: Vec<char> = token.chars().collect();

                let obj = match chars[..] {
                    [] => TObject::Empty,
                    ['(', .., ')'] => {
                        let inner_list = self.parse(token)?;
                        TObject::List(inner_list)
                    }
                    _ => TObject::Symbol(token.into()),
                };

                Ok(obj)
            })
            .collect()
    }

    pub fn tokenize(&mut self, text: &String) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();

        let mut buffer: Vec<char> = Vec::new();
        let mut depth = 0;

        for ch in text.chars() {
            if depth == 0 && self.is_white_space(ch) {
                if !buffer.is_empty() {
                    tokens.push(buffer.iter().collect());
                    buffer.clear();
                }
            } else {
                if ch == '(' {
                    depth += 1;
                }

                if ch == ')' {
                    depth -= 1;
                }

                buffer.push(ch);
            }
        }

        if !buffer.is_empty() {
            tokens.push(buffer.iter().collect());
            buffer.clear();
        }

        return tokens;
    }

    fn is_white_space(&self, ch: char) -> bool {
        let whitespace_chars = [' ', '\t', '\n'];
        return whitespace_chars.contains(&ch);
    }

    fn trim_expression(&self, expr: &String) -> RuntimeResult<String> {
        let char_buffer: Vec<_> = expr.chars().collect();

        let mut new_start = 0;
        while self.is_white_space(char_buffer[new_start]) {
            new_start += 1;
        }

        let mut new_end = char_buffer.len() - 1;
        while self.is_white_space(char_buffer[new_end]) {
            new_end -= 1;
        }

        let trimmed_buffer = &char_buffer[new_start..new_end + 1];
        match &trimmed_buffer {
            ['(', inside @ .., ')'] => Ok(inside.iter().collect()),
            [] => Ok(String::from("")),
            _ => {
                Err(format!("`{}` is not an expression", expr))
            }
        }
    }
}
