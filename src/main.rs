use crate::interpreter::ast::Ast;
use crate::interpreter::tokens::Tokens;
use crate::interpreter::Interpreter;
use std::str::FromStr;

mod interpreter;

fn main() {
    let src = "値　＝　１０．５
表示　値
値　＝　１
表示　値";
    let tokens = Tokens::from_str(src).unwrap();
    let ast = Ast::try_from(tokens).unwrap();
    Interpreter::run(ast).unwrap();
}
