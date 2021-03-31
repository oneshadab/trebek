use crate::{
    misc::RuntimeResult,
    types::{string_literal::TString, symbol::Symbol},
};

use super::types::{list::List, t_object::TObject};

pub struct Parser {
    text: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            text: Vec::new(),
            pos: 0,
        }
    }

    pub fn parse(&mut self, expr: &String) -> RuntimeResult<List> {
        let trimmed_expr = self.trim_expression(expr)?;
        self.text = trimmed_expr.chars().collect();
        self.pos = 0;
        self.next_list()
    }

    fn next_list(&mut self) -> RuntimeResult<List> {
        let mut tokens: List = List::new();

        self.next_char()?;

        while !self.done() {
            let ch = self.peek()?;

            match ch {
                '(' => {
                    let inner_list = self.next_list()?;
                    tokens.push(TObject::List(inner_list));
                }
                ')' => {
                    self.next_char()?;
                    break;
                }
                '"' => {
                    let inner_string = self.next_string()?;
                    tokens.push(TObject::String(inner_string));
                }
                ch if ch.is_whitespace() => {
                    self.next_char()?;
                }
                _ => {
                    let sym = self.next_symbol()?;
                    tokens.push(TObject::Symbol(sym));
                }
            }
        }

        Ok(tokens)
    }

    fn next_string(&mut self) -> RuntimeResult<TString> {
        let mut chars: Vec<char> = Vec::new();

        self.next_char()?;

        while !self.done() {
            let ch = self.peek()?;

            if ch == '"' {
                self.next_char()?;
                break;
            }

            chars.push(ch);

            self.next_char()?;
        }

        let s = chars.iter().collect();
        Ok(s)
    }

    fn next_symbol(&mut self) -> RuntimeResult<Symbol> {
        let mut chars: Vec<char> = Vec::new();

        while !self.done() {
            let ch = self.peek()?;

            if ch.is_whitespace() || ch == '(' || ch == ')' {
                break;
            }

            chars.push(ch);

            self.next_char()?;
        }

        let sym = chars.iter().collect();
        Ok(sym)
    }

    fn next_char(&mut self) -> RuntimeResult<char> {
        let ch = self.peek()?;
        self.pos += 1;
        Ok(ch)
    }

    fn peek(&self) -> RuntimeResult<char> {
        match self.text.get(self.pos) {
            Some(ch) => Ok(ch.clone()),
            None => Err(format!("Token out of bounds")),
        }
    }

    fn done(&self) -> bool {
        self.pos >= self.text.len()
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

        let trimmed_buffer = &char_buffer[new_start..new_end];
        Ok(trimmed_buffer.iter().collect())
    }
}
