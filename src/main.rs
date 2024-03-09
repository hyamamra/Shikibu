use crate::interpreter::ast::Ast;
use crate::interpreter::tokens::Tokens;
use crate::interpreter::Interpreter;
use std::fs::read_to_string;
use std::str::FromStr;

mod interpreter;

fn main() {
    let src = read_to_string("./Shikibu/main.skb").unwrap();
    let tokens = Tokens::from_str(src.as_str()).unwrap();
    let ast = Ast::try_from(tokens).unwrap();
    Interpreter::run(ast).unwrap();
}
