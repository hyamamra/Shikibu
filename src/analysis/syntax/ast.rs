use super::{node::Node, parser::Parser};
use crate::analysis::{error::SyntaxError, lexical::tokens::Tokens};

#[derive(Debug)]
pub struct Ast {
    pub children: Vec<Node>,
}

impl TryFrom<Tokens> for Ast {
    type Error = SyntaxError;

    fn try_from(tokens: Tokens) -> Result<Self, Self::Error> {
        let parser = Parser::new(tokens);
        let children = parser.collect::<Result<Vec<Node>, SyntaxError>>().unwrap();
        Ok(Self { children })
    }
}
