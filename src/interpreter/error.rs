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
    pub node: Option<Node>,
}

impl RuntimeError {
    pub fn unexpected_node(node: Node) -> Self {
        Self {
            message: format!("Unexpected node: {:?}", node),
            node: Some(node),
        }
    }

    pub fn string_addition(node: Node) -> Self {
        Self {
            message: "Cannot add strings".to_string(),
            node: Some(node),
        }
    }

    pub fn comparing_different_types(node: Node) -> Self {
        Self {
            message: "Cannot compare different types".to_string(),
            node: Some(node),
        }
    }

    pub fn undefined_variable(node: Node) -> Self {
        Self {
            message: "Undefined variable".to_string(),
            node: Some(node),
        }
    }

    pub fn undefined_function(node: Node) -> Self {
        Self {
            message: "Undefined function".to_string(),
            node: Some(node),
        }
    }

    pub fn wrong_number_of_arguments(node: Node) -> Self {
        Self {
            message: "Wrong number of arguments".to_string(),
            node: Some(node),
        }
    }
}
