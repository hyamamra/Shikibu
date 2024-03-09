use crate::analysis::ast::Ast;
use crate::analysis::tokens::Tokens;
use std::str::FromStr;

mod analysis;

fn main() {
    let src = "関数　否定（真偽値）
　　もどす　1２＋４
　　もし　真＝＝偽　なら
　　　　くりかえし
　　　　　　表示　「ループ中」
　　　　　　ぬける
　　値＝無
";
    let src = "関数　否定（）
    もどす　値（１,a(b())）";
    let tokens = Tokens::from_str(src).unwrap();
    let ast = Ast::try_from(tokens).unwrap();
    println!("{:#?}", &ast);
}
