use super::{scanner::Scanner, token::Token};
use std::str::FromStr;

#[derive(Debug)]
pub struct Tokens {
    vec: Vec<Token>,
    cursor: usize,
}

impl FromStr for Tokens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanner = Scanner::new(s);
        let mut vec = Vec::new();

        for token in scanner {
            vec.push(token);
        }
        Ok(Self { vec, cursor: 0 })
    }
}
