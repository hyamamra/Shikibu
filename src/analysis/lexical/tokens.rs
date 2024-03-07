use super::{sanitizer::sanitize, scanner::Scanner, token::Token};
use std::str::FromStr;

#[derive(Debug)]
pub struct Tokens(Vec<Token>);

impl FromStr for Tokens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(sanitize(&mut Scanner::new(s).collect()).unwrap()))
    }
}
