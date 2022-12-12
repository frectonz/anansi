use md_parser::{Builder, HeaderLevel, Lexer, Line, Parser, Token};

#[test]
fn parse_header() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);

    lexer.lex("# H1");
    lexer.lex("## H2");
    lexer.lex("### H3");
    lexer.lex("#### H4");
    lexer.lex("##### H5");
    lexer.lex("###### H6");

    use HeaderLevel::*;
    use Token::*;
    assert_eq!(
        builder.get_document(),
        vec![
            Line::Header {
                level: H1,
                tokens: vec![Regular("H1".into())]
            },
            Line::Header {
                level: H2,
                tokens: vec![Regular("H2".into())]
            },
            Line::Header {
                level: H3,
                tokens: vec![Regular("H3".into())]
            },
            Line::Header {
                level: H4,
                tokens: vec![Regular("H4".into())]
            },
            Line::Header {
                level: H5,
                tokens: vec![Regular("H5".into())]
            },
            Line::Header {
                level: H6,
                tokens: vec![Regular("H6".into())]
            }
        ]
    );
}

#[test]
fn parse_bold() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);

    lexer.lex("**bold**");
    lexer.lex("regular **bold** word");
    lexer.lex("and __another__ bold word");
    lexer.lex("**bold with spaces**");

    assert_eq!(
        builder.get_document(),
        &[
            Line::Paragraph(vec![Token::Bold(vec![Token::Regular("bold".into())])]),
            Line::Paragraph(vec![
                Token::Regular("regular".into()),
                Token::Bold(vec![Token::Regular("bold".into())]),
                Token::Regular("word".into())
            ]),
            Line::Paragraph(vec![
                Token::Regular("and".to_string()),
                Token::Bold(vec![Token::Regular("another".to_string())]),
                Token::Regular("bold".to_string()),
                Token::Regular("word".to_string())
            ]),
            Line::Paragraph(vec![Token::Bold(vec![
                Token::Regular("bold".to_string()),
                Token::Regular("with".to_string()),
                Token::Regular("spaces".to_string())
            ])]),
        ]
    );
}

#[test]
fn parse_italic() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);

    lexer.lex("*italic*");
    lexer.lex("regular *italic* word");
    lexer.lex("and _another_ italic word");
    lexer.lex("*italic with spaces*");

    assert_eq!(
        builder.get_document(),
        &[
            Line::Paragraph(vec![Token::Italic(vec![Token::Regular(
                "italic".to_string()
            )])]),
            Line::Paragraph(vec![
                Token::Regular("regular".to_string()),
                Token::Italic(vec![Token::Regular("italic".to_string())]),
                Token::Regular("word".to_string())
            ]),
            Line::Paragraph(vec![
                Token::Regular("and".to_string()),
                Token::Italic(vec![Token::Regular("another".to_string())]),
                Token::Regular("italic".to_string()),
                Token::Regular("word".to_string())
            ]),
            Line::Paragraph(vec![Token::Italic(vec![
                Token::Regular("italic".to_string()),
                Token::Regular("with".to_string()),
                Token::Regular("spaces".to_string())
            ])]),
        ]
    );
}

#[test]
fn parse_link() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);

    lexer.lex("a regular [Link](https://a.com)");
    lexer.lex("and [Another Link](https://b.com) with spaces");

    assert_eq!(
        builder.get_document(),
        &[
            Line::Paragraph(vec![
                Token::Regular("a".to_string()),
                Token::Regular("regular".to_string()),
                Token::Link {
                    label: vec![Token::Regular("Link".to_string())],
                    url: "https://a.com".to_string()
                }
            ]),
            Line::Paragraph(vec![
                Token::Regular("and".to_string()),
                Token::Link {
                    label: vec![
                        Token::Regular("Another".to_string()),
                        Token::Regular("Link".to_string())
                    ],
                    url: "https://b.com".to_string()
                },
                Token::Regular("with".to_string()),
                Token::Regular("spaces".to_string())
            ])
        ]
    );
}

#[test]
fn parse_bold_link() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);

    lexer.lex("**[Bold](https://a.com)**");

    assert_eq!(
        builder.get_document(),
        &[Line::Paragraph(vec![Token::Bold(vec![Token::Link {
            label: vec![Token::Regular("Bold".to_string()),],
            url: "https://a.com".to_string()
        }])])]
    );
}

#[test]
fn lex_image() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);
    lexer.lex("![image](https://www.a.com)");

    assert_eq!(
        builder.get_document(),
        vec![Line::Image {
            label: vec![Token::Regular("image".into())],
            url: "https://www.a.com".to_string()
        }]
    );
}

#[test]
fn parse_inline_code() {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    let mut lexer = Lexer::new(&mut parser);

    lexer.lex("regular `code` word");
    lexer.lex("a `code with spaces`.");

    use Line::*;
    use Token::*;
    assert_eq!(
        builder.get_document(),
        vec![
            Paragraph(
                [
                    Regular("regular".into()),
                    InlineCode([Regular("code".into())].to_vec()),
                    Regular("word".into())
                ]
                .to_vec()
            ),
            Paragraph(
                [
                    Regular("a".into()),
                    InlineCode(
                        [
                            Regular("code".into()),
                            Regular("with".into()),
                            Regular("spaces.".into())
                        ]
                        .to_vec()
                    )
                ]
                .to_vec()
            )
        ]
    );
}
