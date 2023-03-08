mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("1_9");
    let tokens = lexer.lex();

    // println!("{tokens:?}");
}
