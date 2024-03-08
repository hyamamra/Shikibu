use crate::analysis::lexical::tokens::Tokens;
use crate::analysis::syntax::ast::Ast;
use std::str::FromStr;

mod analysis;

fn main() {
    let src = "関数　否定（真偽値）
　　もどす　1
　　もし　１　なら
　　　　くりかえし
　　　　　　表示　２
　　　　　　ぬける";
    let tokens = Tokens::from_str(src).unwrap();
    let ast = Ast::try_from(tokens).unwrap();
    println!("{:#?}", &ast);
}
