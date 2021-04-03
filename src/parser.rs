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

    pub fn parse(&mut self, program: &String) -> RuntimeResult<Vec<TObject>> {
        self.text = program.chars().collect();
        self.pos = 0;

        let mut exprs = Vec::new();
        loop {
            self.skip_whitespace()?;
            if self.done() {
                break;
            }

            exprs.push(self.next()?);
        }
        Ok(exprs)
    }

    fn next(&mut self) -> RuntimeResult<TObject> {
        let ch = self.peek()?;

        let obj = match ch {
            '(' => TObject::List(self.next_list()?),
            '[' => {
                let mut list = self.next_list()?;

                list.push_front(TObject::Symbol("list".into()));

                TObject::List(list)
            }
            '{' => {
                let mut dict = self.next_list()?;
                dict.push_front(TObject::Symbol("dict".into()));

                TObject::List(dict)
            }
            '\'' => {
                self.next_char()?;

                let mut list = self.next_list()?;
                list.push_front(TObject::Symbol("quote".into()));

                TObject::List(list)
            }
            '"' => TObject::String(self.next_string()?),
            _ => TObject::Symbol(self.next_symbol()?),
        };

        Ok(obj)
    }

    fn next_list(&mut self) -> RuntimeResult<List> {
        let mut tokens: List = List::new();

        self.next_char()?;

        loop {
            self.skip_whitespace()?;
            if self.done() {
                break;
            }

            let ch = self.peek()?;
            if [')', ']', '}'].contains(&ch) {
                self.next_char()?;
                break;
            }

            tokens.push_back(self.next()?);
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

            chars.push(self.next_char()?);
        }

        let s = chars.iter().collect();
        Ok(s)
    }

    fn next_symbol(&mut self) -> RuntimeResult<Symbol> {
        let mut chars: Vec<char> = Vec::new();

        while !self.done() {
            let ch = self.peek()?;

            if ch.is_whitespace() || ['(', ')', '[', ']', '{', '}'].contains(&ch) {
                break;
            }

            chars.push(self.next_char()?);
        }

        let sym = chars.iter().collect();
        Ok(sym)
    }

    fn next_char(&mut self) -> RuntimeResult<char> {
        let ch = self.peek()?;
        self.pos += 1;
        Ok(ch)
    }

    fn skip_whitespace(&mut self) -> RuntimeResult<()> {
        while !self.done() {
            let ch = self.peek()?;

            if !ch.is_whitespace() {
                break;
            }

            self.next_char()?;
        }
        Ok(())
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
}
