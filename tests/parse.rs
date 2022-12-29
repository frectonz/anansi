use md_parser::{Builder, HeaderLevel, Line, Parser, Token};

fn parse(input: &str) -> Vec<Line> {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    parser.parse(input);
    builder.get_document()
}

#[test]
fn parse_header() {
    use HeaderLevel::*;
    use Token::*;
    assert_eq!(
        parse("# H1"),
        vec![Line::Header {
            level: H1,
            tokens: vec![Regular("H1".into())]
        }]
    );

    assert_eq!(
        parse("## H2"),
        vec![Line::Header {
            level: H2,
            tokens: vec![Regular("H2".into())]
        }]
    );

    assert_eq!(
        parse("### H3"),
        vec![Line::Header {
            level: H3,
            tokens: vec![Regular("H3".into())]
        }]
    );
}

#[test]
fn parse_bold() {
    use Token::*;
    assert_eq!(
        parse("**bold**"),
        &[Line::Paragraph(vec![Bold(vec![Regular(
            "bold".to_string()
        )])])]
    );
}

#[test]
fn parse_italic() {
    use Token::*;
    assert_eq!(
        parse("*italic*"),
        &[Line::Paragraph(vec![Italic(vec![Regular(
            "italic".to_string()
        )])])]
    );
}

#[test]
#[ignore]
fn parse_link() {
    use Token::*;
    assert_eq!(
        parse("[Link](https://a.com)"),
        &[Line::Paragraph(vec![Link {
            label: vec![Token::Regular("Link".to_string())],
            url: "https://a.com".to_string()
        }])]
    );
}

#[test]
#[ignore]
fn parse_bold_link() {
    use Token::*;
    assert_eq!(
        parse("**[Bold](https://a.com)**"),
        &[Line::Paragraph(vec![Bold(vec![Link {
            label: vec![Regular("Bold".to_string()),],
            url: "https://a.com".to_string()
        }])])]
    );
}

#[test]
#[ignore]
fn parse_bold_italic() {
    use Token::*;
    assert_eq!(
        parse("***strong emph***"),
        vec![Line::Paragraph(vec![Bold(vec![Italic(vec![
            Regular("strong".into()),
            Regular("emph".into())
        ])])])]
    );
}
