use super::{ast::node::Node, tokens::token::Token};

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub token: Option<Token>,
}

impl SyntaxError {
    pub fn invalid_char(c: char, position: usize) -> Self {
        Self {
            message: format!("Invalid character: {}", c),
            token: Some(Token::invalid(c, position)),
        }
    }

    pub fn unexpected_token(token: Token) -> Self {
        Self {
            message: format!("Unexpected token: {:?}", token.lexeme),
            token: Some(token),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub token: Option<Node>,
}

impl RuntimeError {
    pub fn unexpected_node(node: Node) -> Self {
        Self {
            message: format!("Unexpected node: {:?}", node),
            token: Some(node),
        }
    }
}
