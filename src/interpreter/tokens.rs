use self::{lexeme::Lexeme, sanitizer::sanitize, scanner::Scanner, token::Token};
use std::{collections::VecDeque, str::FromStr};

pub mod keyword;
pub mod lexeme;
mod sanitizer;
mod scanner;
pub mod symbol;
pub mod token;

#[derive(Debug)]
pub struct Tokens {
    queue: VecDeque<Token>,
}

impl FromStr for Tokens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanner = Scanner::new(s);
        let mut q = scanner.collect();
        let queue = sanitize(&mut q).unwrap();
        Ok(Self { queue })
    }
}

impl Tokens {
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn front(&self) -> Option<&Token> {
        self.queue.front()
    }

    pub fn shift(&mut self) -> Option<Token> {
        self.queue.pop_front()
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.queue.get(index)
    }

    pub fn consume(&mut self, lexeme: Lexeme) -> Result<(), ()> {
        if let Some(token) = self.front() {
            if token.lexeme == lexeme {
                _ = self.queue.pop_front();
                return Ok(());
            }
        }
        Err(())
    }
}
