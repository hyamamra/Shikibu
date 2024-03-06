use analysis::lexical::tokens::Tokens;
use std::str::FromStr;

mod analysis;

fn main() {
    let tokens = Tokens::from_str("値＝１").unwrap();
    println!("{:#?}", &tokens);
}
