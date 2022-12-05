use crate::{HeaderLevel, Line, Token, TokenCollector};

pub struct Parser {
    words: Vec<Token>,
    header_level: Option<HeaderLevel>,
    parsing_bold: bool,
    parsing_italic: bool,
    parsing_link: bool,
    tokens: Vec<Line>,
}

impl TokenCollector for Parser {
    fn h1(&mut self) {
        self.header_level = Some(HeaderLevel::H1);
    }

    fn h2(&mut self) {
        self.header_level = Some(HeaderLevel::H2);
    }

    fn h3(&mut self) {
        self.header_level = Some(HeaderLevel::H3);
    }

    fn h4(&mut self) {
        self.header_level = Some(HeaderLevel::H4);
    }

    fn h5(&mut self) {
        self.header_level = Some(HeaderLevel::H5);
    }

    fn h6(&mut self) {
        self.header_level = Some(HeaderLevel::H6);
    }

    fn begin_bold(&mut self) {
        self.parsing_bold = true;
        self.words.push(Token::Bold(vec![]));
    }

    fn end_bold(&mut self) {
        self.parsing_bold = false;
    }

    fn begin_italic(&mut self) {
        self.parsing_italic = true;
        self.words.push(Token::Italic(vec![]));
    }

    fn end_italic(&mut self) {
        self.parsing_italic = false;
    }

    fn begin_label(&mut self) {
        self.parsing_link = true;
        self.words.push(Token::Link {
            label: vec![],
            url: String::new(),
        });
    }

    fn end_label(&mut self) {}

    fn url(&mut self, url: &str) {
        if self.parsing_link {
            if let Some(Token::Link { url: ref mut u, .. }) = self.words.last_mut() {
                *u = url.to_string();
            }
            self.parsing_link = false;
        }
    }

    fn word(&mut self, word: &str) {
        if self.parsing_bold {
            if let Some(Token::Bold(ref mut tokens)) = self.words.last_mut() {
                tokens.push(Token::Regular(word.to_string()));
            }
        } else if self.parsing_italic {
            if let Some(Token::Italic(ref mut tokens)) = self.words.last_mut() {
                tokens.push(Token::Regular(word.to_string()));
            }
        } else if self.parsing_link {
            if let Some(Token::Link {
                label: ref mut tokens,
                ..
            }) = self.words.last_mut()
            {
                tokens.push(Token::Regular(word.to_string()));
            }
        } else {
            self.words.push(Token::Regular(word.to_string()));
        }
    }

    fn line_break(&mut self) {
        if self.header_level.is_some() {
            let header = Line::Header {
                level: self.header_level.take().unwrap(),
                tokens: self.words.drain(..).collect(),
            };

            self.tokens.push(header);
        } else {
            let text = Line::Text(self.words.drain(..).collect());
            self.tokens.push(text);
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            header_level: None,
            parsing_bold: false,
            parsing_italic: false,
            parsing_link: false,
            tokens: Vec::new(),
        }
    }

    pub fn tokens(&self) -> &[Line] {
        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::{HeaderLevel, Line, Parser, Token};
    use crate::Lexer;

    #[test]
    fn parse_header() {
        let mut parser = Parser::new();
        let mut lexer = Lexer::new(&mut parser);

        lexer.lex("# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6");

        assert_eq!(
            parser.tokens(),
            &[
                Line::Header {
                    level: HeaderLevel::H1,
                    tokens: vec![Token::Regular("H1".to_string())],
                },
                Line::Header {
                    level: HeaderLevel::H2,
                    tokens: vec![Token::Regular("H2".to_string())],
                },
                Line::Header {
                    level: HeaderLevel::H3,
                    tokens: vec![Token::Regular("H3".to_string())],
                },
                Line::Header {
                    level: HeaderLevel::H4,
                    tokens: vec![Token::Regular("H4".to_string())],
                },
                Line::Header {
                    level: HeaderLevel::H5,
                    tokens: vec![Token::Regular("H5".to_string())],
                },
                Line::Header {
                    level: HeaderLevel::H6,
                    tokens: vec![Token::Regular("H6".to_string())],
                },
            ]
        );
    }

    #[test]
    fn parse_bold() {
        let mut parser = Parser::new();
        let mut lexer = Lexer::new(&mut parser);

        lexer.lex("**bold**");
        lexer.lex("regular **bold** word");
        lexer.lex("and __another__ bold word");
        lexer.lex("**bold with spaces**");

        assert_eq!(
            parser.tokens(),
            &[
                Line::Text(vec![Token::Bold(vec![Token::Regular("bold".to_string())])]),
                Line::Text(vec![
                    Token::Regular("regular".to_string()),
                    Token::Bold(vec![Token::Regular("bold".to_string())]),
                    Token::Regular("word".to_string())
                ]),
                Line::Text(vec![
                    Token::Regular("and".to_string()),
                    Token::Bold(vec![Token::Regular("another".to_string())]),
                    Token::Regular("bold".to_string()),
                    Token::Regular("word".to_string())
                ]),
                Line::Text(vec![Token::Bold(vec![
                    Token::Regular("bold".to_string()),
                    Token::Regular("with".to_string()),
                    Token::Regular("spaces".to_string())
                ])]),
            ]
        );
    }

    #[test]
    fn parse_italic() {
        let mut parser = Parser::new();
        let mut lexer = Lexer::new(&mut parser);

        lexer.lex("*italic*");
        lexer.lex("regular *italic* word");
        lexer.lex("and _another_ italic word");
        lexer.lex("*italic with spaces*");

        assert_eq!(
            parser.tokens(),
            &[
                Line::Text(vec![Token::Italic(vec![Token::Regular(
                    "italic".to_string()
                )])]),
                Line::Text(vec![
                    Token::Regular("regular".to_string()),
                    Token::Italic(vec![Token::Regular("italic".to_string())]),
                    Token::Regular("word".to_string())
                ]),
                Line::Text(vec![
                    Token::Regular("and".to_string()),
                    Token::Italic(vec![Token::Regular("another".to_string())]),
                    Token::Regular("italic".to_string()),
                    Token::Regular("word".to_string())
                ]),
                Line::Text(vec![Token::Italic(vec![
                    Token::Regular("italic".to_string()),
                    Token::Regular("with".to_string()),
                    Token::Regular("spaces".to_string())
                ])]),
            ]
        );
    }

    #[test]
    fn parse_link() {
        let mut parser = Parser::new();
        let mut lexer = Lexer::new(&mut parser);

        lexer.lex("a regular [Link](https://a.com)");
        lexer.lex("and [Another Link](https://b.com) with spaces");

        assert_eq!(
            parser.tokens(),
            &[
                Line::Text(vec![
                    Token::Regular("a".to_string()),
                    Token::Regular("regular".to_string()),
                    Token::Link {
                        label: vec![Token::Regular("Link".to_string())],
                        url: "https://a.com".to_string()
                    }
                ]),
                Line::Text(vec![
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
    #[ignore]
    fn parse_bold_link() {
        let mut parser = Parser::new();
        let mut lexer = Lexer::new(&mut parser);

        lexer.lex("**[Bold](https://a.com)**");

        assert_eq!(
            parser.tokens(),
            &[Line::Text(vec![Token::Bold(vec![Token::Link {
                label: vec![Token::Regular("Bold".to_string()),],
                url: "https://a.com".to_string()
            }])])]
        );
    }
}
