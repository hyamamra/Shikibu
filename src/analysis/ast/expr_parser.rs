use super::node::Node;
use crate::analysis::{
    error::SyntaxError,
    tokens::{keyword::Keyword, lexeme::Lexeme, symbol::Symbol, Tokens},
};

pub fn parse_expression(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    parse_or(tokens)
}

fn parse_or(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let mut node = parse_and(tokens).unwrap();

    loop {
        if tokens.consume(Lexeme::Keyword(Keyword::Or)).is_ok() {
            node = Node::Or {
                left: Box::new(node),
                right: Box::new(parse_and(tokens).unwrap()),
            };
        } else {
            return Ok(node);
        }
    }
}

fn parse_and(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let mut node = parse_equality(tokens).unwrap();

    loop {
        if tokens.consume(Lexeme::Keyword(Keyword::And)).is_ok() {
            node = Node::And {
                left: Box::new(node),
                right: Box::new(parse_equality(tokens).unwrap()),
            };
        } else {
            return Ok(node);
        }
    }
}

fn parse_equality(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let mut node = parse_relational(tokens).unwrap();

    loop {
        if tokens.consume(Lexeme::Symbol(Symbol::EqualEqual)).is_ok() {
            node = Node::Equal {
                left: Box::new(node),
                right: Box::new(parse_relational(tokens).unwrap()),
            };
        } else if tokens.consume(Lexeme::Symbol(Symbol::BangEqual)).is_ok() {
            node = Node::NotEqual {
                left: Box::new(node),
                right: Box::new(parse_relational(tokens).unwrap()),
            };
        } else {
            return Ok(node);
        }
    }
}

fn parse_relational(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let mut node = parse_add_or_sub(tokens).unwrap();

    loop {
        if tokens.consume(Lexeme::Symbol(Symbol::Less)).is_ok() {
            node = Node::LessThan {
                left: Box::new(node),
                right: Box::new(parse_add_or_sub(tokens).unwrap()),
            };
        } else if tokens.consume(Lexeme::Symbol(Symbol::LessEqual)).is_ok() {
            node = Node::LessThanOrEqual {
                left: Box::new(node),
                right: Box::new(parse_add_or_sub(tokens).unwrap()),
            };
        } else if tokens.consume(Lexeme::Symbol(Symbol::Greater)).is_ok() {
            node = Node::GreaterThan {
                left: Box::new(node),
                right: Box::new(parse_add_or_sub(tokens).unwrap()),
            };
        } else if tokens.consume(Lexeme::Symbol(Symbol::GreaterEqual)).is_ok() {
            node = Node::GreaterThanOrEqual {
                left: Box::new(node),
                right: Box::new(parse_add_or_sub(tokens).unwrap()),
            };
        } else {
            return Ok(node);
        }
    }
}

fn parse_add_or_sub(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let mut node = parse_mul_or_div(tokens).unwrap();

    loop {
        if tokens.consume(Lexeme::Symbol(Symbol::Plus)).is_ok() {
            node = Node::Add {
                left: Box::new(node),
                right: Box::new(parse_mul_or_div(tokens).unwrap()),
            };
        } else if tokens.consume(Lexeme::Symbol(Symbol::Minus)).is_ok() {
            node = Node::Subtract {
                left: Box::new(node),
                right: Box::new(parse_mul_or_div(tokens).unwrap()),
            };
        } else {
            return Ok(node);
        }
    }
}

fn parse_mul_or_div(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let mut node = parse_negate(tokens).unwrap();

    loop {
        if tokens.consume(Lexeme::Symbol(Symbol::Asterisk)).is_ok() {
            node = Node::Multiply {
                left: Box::new(node),
                right: Box::new(parse_negate(tokens).unwrap()),
            };
        } else if tokens.consume(Lexeme::Symbol(Symbol::Slash)).is_ok() {
            node = Node::Divide {
                left: Box::new(node),
                right: Box::new(parse_negate(tokens).unwrap()),
            };
        } else {
            return Ok(node);
        }
    }
}

fn parse_negate(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    if tokens.consume(Lexeme::Symbol(Symbol::Minus)).is_ok() {
        Ok(Node::Subtract {
            left: Box::new(Node::Number("0".to_string())),
            right: Box::new(parse_primary(tokens).unwrap()),
        })
    } else {
        parse_primary(tokens)
    }
}

fn parse_primary(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    if tokens.consume(Lexeme::Symbol(Symbol::OpenParen)).is_ok() {
        let node = parse_expression(tokens).unwrap();
        tokens.consume(Lexeme::Symbol(Symbol::CloseParen)).unwrap();
        Ok(node)
    } else {
        parse_terminal(tokens)
    }
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
