use self::{node::Node, parser::Parser};
use super::{error::SyntaxError, tokens::Tokens};

mod expr_parser;
mod node;
mod parser;

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
