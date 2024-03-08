use super::node::Node;
use crate::analysis::{
    error::SyntaxError,
    lexical::{lexeme::Lexeme, tokens::Tokens},
};

pub fn parse_expression(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    parse_number(tokens)
}

fn parse_number(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let token = tokens.shift().unwrap();
    if let Lexeme::Number(number) = token.lexeme {
        Ok(Node::Number(number))
    } else {
        Err(SyntaxError::unexpected_token(token))
    }
}
