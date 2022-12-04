use md_parser::{Lexer, Parser};

fn main() {
    let mut parser = Parser::new();
    let mut lexer = Lexer::new(&mut parser);

    let content = std::fs::read_to_string("TEST.md").unwrap();

    lexer.lex(&content);

    dbg!(parser.tokens());
}
