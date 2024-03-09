use std::collections::VecDeque;

use self::{node::Node, parser::Parser};
use super::{error::SyntaxError, tokens::Tokens};

mod expr_parser;
pub mod node;
mod parser;

#[derive(Debug)]
pub struct Ast {
    children: VecDeque<Node>,
}

impl TryFrom<Tokens> for Ast {
    type Error = SyntaxError;

    fn try_from(tokens: Tokens) -> Result<Self, Self::Error> {
        let parser = Parser::new(tokens);
        let children = parser
            .collect::<Result<VecDeque<Node>, SyntaxError>>()
            .unwrap();
        Ok(Self { children })
    }
}

impl Ast {
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn pop_front(&mut self) -> Option<Node> {
        self.children.pop_front()
    }

    pub fn drain_functions(&mut self) -> Vec<Node> {
        let mut functions = Vec::new();
        self.children.retain(|node| match node {
            Node::Function { .. } => {
                functions.push(node.clone());
                false
            }
            _ => true,
        });
        functions
    }
}
