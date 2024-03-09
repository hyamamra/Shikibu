use super::node::Node;
use crate::analysis::{
    error::SyntaxError,
    tokens::{keyword::Keyword, lexeme::Lexeme, symbol::Symbol, Tokens},
};

pub fn parse_expression(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    parse_terminal(tokens)
}

fn parse_terminal(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let token = tokens.shift().unwrap();
    match token.lexeme {
        Lexeme::Identifier(name) => {
            if tokens.consume(Lexeme::Symbol(Symbol::OpenParen)).is_ok() {
                let mut args = Vec::new();
                loop {
                    if tokens.consume(Lexeme::Symbol(Symbol::CloseParen)).is_ok() {
                        break;
                    }
                    if tokens.consume(Lexeme::Symbol(Symbol::Comma)).is_err() {
                        args.push(parse_expression(tokens).unwrap());
                    }
                }
                Ok(Node::Call { name, args })
            } else {
                Ok(Node::Variable(name))
            }
        }
        Lexeme::Number(number) => Ok(Node::Number(number)),
        Lexeme::String(string) => Ok(Node::String(string)),
        Lexeme::Keyword(Keyword::Null) => Ok(Node::Null),
        Lexeme::Keyword(Keyword::True) => Ok(Node::True),
        Lexeme::Keyword(Keyword::False) => Ok(Node::False),
        _ => Err(SyntaxError::unexpected_token(token)),
    }
}
