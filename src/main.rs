use md_parser::{Builder, Lexer, Parser};

fn main() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);

    let content = std::fs::read_to_string("TEST.md").unwrap();

    lexer.lex(&content);

    dbg!(builder.get_document());
}
