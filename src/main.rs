use md_parser::{Builder, Parser};

fn main() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);

    let content = std::fs::read_to_string("TEST.md").unwrap();
    parser.parse(&content);

    dbg!(builder.get_document());
}
