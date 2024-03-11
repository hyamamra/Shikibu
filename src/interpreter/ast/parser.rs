use super::node::Node;
use crate::interpreter::{
    ast::expr_parser::parse_expression,
    error::SyntaxError,
    tokens::{keyword::Keyword, lexeme::Lexeme, symbol::Symbol, Tokens},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Tokens,
}

impl Iterator for Parser {
    type Item = Result<Node, SyntaxError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.front().is_none() {
            return None;
        };
        Some(parse_node(&mut self.tokens))
    }
}

impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self { tokens }
    }
}

fn parse_node(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let front = tokens.front().unwrap();

    let node = match &front.lexeme {
        Lexeme::Keyword(keyword) => match keyword {
            Keyword::If => parse_if(tokens),
            Keyword::Function => parse_function(tokens),
            Keyword::Return => parse_return(tokens),
            Keyword::Loop => parse_loop(tokens),
            Keyword::Continue => parse_continue(tokens),
            Keyword::Break => parse_break(tokens),
            Keyword::Print => parse_print(tokens),
            _ => Err(SyntaxError::unexpected_token(front.clone())),
        },
        Lexeme::Identifier(_) => {
            if is_assignment(tokens) {
                parse_assignment(tokens)
            } else {
                parse_expression(tokens)
            }
        }
        _ => Err(SyntaxError::unexpected_token(front.clone())),
    };

    tokens.consume(Lexeme::Newline).ok();
    node
}

fn parse_node_in_block(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let front = tokens.front().unwrap();

    let node = match &front.lexeme {
        Lexeme::Keyword(keyword) => match keyword {
            Keyword::If => parse_if(tokens),
            Keyword::Function => Err(SyntaxError::function_declaration_in_block(front.clone())),
            Keyword::Return => parse_return(tokens),
            Keyword::Loop => parse_loop(tokens),
            Keyword::Continue => parse_continue(tokens),
            Keyword::Break => parse_break(tokens),
            Keyword::Print => parse_print(tokens),
            _ => Err(SyntaxError::unexpected_token(front.clone())),
        },
        Lexeme::Identifier(_) => {
            if is_assignment(tokens) {
                parse_assignment(tokens)
            } else {
                parse_expression(tokens)
            }
        }
        _ => Err(SyntaxError::unexpected_token(front.clone())),
    };

    tokens.consume(Lexeme::Newline).ok();
    node
}

fn is_assignment(tokens: &Tokens) -> bool {
    for i in 0..tokens.len() {
        match tokens.get(i).unwrap().lexeme {
            Lexeme::Symbol(Symbol::Equal) => return true,
            Lexeme::Newline => return false,
            _ => continue,
        }
    }
    false
}

fn parse_block(tokens: &mut Tokens) -> Result<Vec<Node>, SyntaxError> {
    tokens.consume(Lexeme::Indent).unwrap();

    let mut body = Vec::new();
    while tokens.consume(Lexeme::Dedent).is_err() {
        body.push(parse_node_in_block(tokens).unwrap());
    }
    Ok(body)
}

pub fn parse_assignment(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    let front = tokens.shift().unwrap();
    if let Lexeme::Identifier(name) = front.lexeme {
        // Assign a value to an index of an array.
        if tokens.consume(Lexeme::Symbol(Symbol::OpenBracket)).is_ok() {
            let index = Box::new(parse_expression(tokens).unwrap());
            tokens
                .consume(Lexeme::Symbol(Symbol::CloseBracket))
                .unwrap();
            tokens.consume(Lexeme::Symbol(Symbol::Equal)).unwrap();
            let value = Box::new(parse_expression(tokens).unwrap());
            return Ok(Node::IndexAssignment { name, index, value });
        }
        // Assign a value to a variable.
        tokens.consume(Lexeme::Symbol(Symbol::Equal)).unwrap();
        let value = Box::new(parse_expression(tokens).unwrap());
        Ok(Node::Assignment { name, value })
    } else {
        Err(SyntaxError::unexpected_token(front))
    }
}

fn parse_function(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    tokens.consume(Lexeme::Keyword(Keyword::Function)).unwrap();

    let token = tokens.shift().unwrap();
    let Lexeme::Identifier(name) = token.lexeme else {
        return Err(SyntaxError::unexpected_token(token));
    };

    tokens.consume(Lexeme::Symbol(Symbol::OpenParen)).unwrap();
    let mut params = Vec::new();
    while let Some(token) = tokens.shift() {
        match token.lexeme {
            Lexeme::Symbol(Symbol::Comma) => continue,
            Lexeme::Symbol(Symbol::CloseParen) => break,
            Lexeme::Identifier(name) => params.push(name),
            _ => return Err(SyntaxError::unexpected_token(token)),
        }
    }
    tokens.consume(Lexeme::Newline).unwrap();
    let body = parse_block(tokens).unwrap();

    Ok(Node::Function { name, params, body })
}

fn parse_if(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    // consumed `もし` or `もしくは`
    _ = tokens.shift().unwrap();
    tokens.consume(Lexeme::Newline).ok();
    let is_indented = tokens.consume(Lexeme::Indent).is_ok();
    let condition = Box::new(parse_expression(tokens).unwrap());
    if is_indented {
        tokens.consume(Lexeme::Dedent).unwrap();
    };

    tokens.consume(Lexeme::Keyword(Keyword::Then)).unwrap();
    tokens.consume(Lexeme::Newline).unwrap();
    let then_part = parse_block(tokens).unwrap();

    if let Some(token) = tokens.front() {
        if token.lexeme == Lexeme::Keyword(Keyword::Elif) {
            return Ok(Node::If {
                condition,
                then_part,
                else_part: vec![parse_if(tokens).unwrap()],
            });
        }
    }

    if tokens.consume(Lexeme::Keyword(Keyword::Else)).is_ok() {
        tokens.consume(Lexeme::Newline).unwrap();
        Ok(Node::If {
            condition,
            then_part,
            else_part: parse_block(tokens).unwrap(),
        })
    } else {
        Ok(Node::If {
            condition,
            then_part,
            else_part: Vec::new(),
        })
    }
}

fn parse_return(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    tokens.consume(Lexeme::Keyword(Keyword::Return)).unwrap();
    if tokens.front().unwrap().lexeme == Lexeme::Newline {
        return Ok(Node::Return(Box::new(Node::Null)));
    }
    let value = parse_expression(tokens).unwrap();
    Ok(Node::Return(Box::new(value)))
}

fn parse_loop(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    tokens.consume(Lexeme::Keyword(Keyword::Loop)).unwrap();
    tokens.consume(Lexeme::Newline).unwrap();
    Ok(Node::Loop {
        body: parse_block(tokens).unwrap(),
    })
}

fn parse_continue(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    tokens.consume(Lexeme::Keyword(Keyword::Continue)).unwrap();
    Ok(Node::Continue)
}

fn parse_break(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    tokens.consume(Lexeme::Keyword(Keyword::Break)).unwrap();
    Ok(Node::Break)
}

fn parse_print(tokens: &mut Tokens) -> Result<Node, SyntaxError> {
    tokens.consume(Lexeme::Keyword(Keyword::Print)).unwrap();
    tokens.consume(Lexeme::Symbol(Symbol::OpenParen)).unwrap();
    let value = parse_expression(tokens).unwrap();
    tokens.consume(Lexeme::Symbol(Symbol::CloseParen)).unwrap();
    Ok(Node::Print(Box::new(value)))
}
