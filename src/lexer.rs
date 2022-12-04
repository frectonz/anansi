use crate::TokenCollector;

pub struct Lexer<'a, T>
where
    T: TokenCollector,
{
    collector: &'a mut T,
}

impl<'a, T> Lexer<'a, T>
where
    T: TokenCollector,
{
    pub fn new(collector: &'a mut T) -> Self {
        Self { collector }
    }

    pub fn lex(&mut self, input: &str) {
        let lines = input.lines();

        for line in lines {
            self.lex_line(line.trim());
            self.collector.line_break();
        }
    }

    fn lex_line(&mut self, line: &str) {
        let first_char = line.chars().next();

        match first_char {
            Some('#') => {
                self.lex_header(line);
            }
            Some(_) => {
                let words = line.split_whitespace();

                for word in words {
                    self.lex_word(word);
                }
            }
            None => {}
        }
    }

    fn lex_header(&mut self, line: &str) {
        let mut words = line.split_whitespace();

        match words.next() {
            Some("#") => self.collector.h1(),
            Some("##") => self.collector.h2(),
            Some("###") => self.collector.h3(),
            Some("####") => self.collector.h4(),
            Some("#####") => self.collector.h5(),
            Some("######") => self.collector.h6(),
            _ => {}
        };

        for word in words {
            self.lex_word(word);
        }
    }

    fn lex_word(&mut self, word: &str) {
        self.lex_bold(word)
            .or_else(|| self.lex_italic(word))
            .or_else(|| self.lex_label(word))
            .unwrap_or_else(|| {
                self.collector.word(word);
            });
    }

    fn lex_bold(&mut self, word: &str) -> Option<()> {
        if word.starts_with("**") || word.starts_with("__") {
            self.collector.begin_bold();
            if word.ends_with("**") || word.ends_with("__") {
                self.collector.word(&word[2..word.len() - 2]);
                self.collector.end_bold();
            } else {
                self.collector.word(&word[2..]);
            }
            return Some(());
        } else if word.ends_with("**") || word.ends_with("__") {
            self.collector.word(&word[..word.len() - 2]);
            self.collector.end_bold();
            return Some(());
        }

        None
    }

    fn lex_italic(&mut self, word: &str) -> Option<()> {
        if word.starts_with("*") || word.starts_with("_") {
            self.collector.begin_italic();
            if word.ends_with("*") || word.ends_with("_") {
                self.collector.word(&word[1..word.len() - 1]);
                self.collector.end_italic();
            } else {
                self.collector.word(&word[1..]);
            }
            Some(())
        } else if word.ends_with("*") || word.ends_with("_") {
            self.collector.word(&word[..word.len() - 1]);
            self.collector.end_italic();
            Some(())
        } else {
            None
        }
    }

    fn lex_label(&mut self, word: &str) -> Option<()> {
        if word.starts_with("[") {
            self.collector.begin_label();

            if word.contains("](") {
                let mut parts = word.split("](");
                let label = parts.next().unwrap_or_default();
                let url = parts.next().unwrap_or_default();

                self.collector.word(&label[1..]);
                self.collector.end_label();
                self.collector.url(&url[..url.len() - 1]);
            } else {
                self.collector.word(&word[1..]);
            }

            return Some(());
        } else if word.contains("]") {
            let mut parts = word.split("]");
            let label = parts.next().unwrap_or_default();
            let url = parts.next().unwrap_or_default();

            self.collector.word(label);
            self.collector.end_label();
            self.collector.url(&url[1..url.len() - 1]);

            return Some(());
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::MockTokenCollector;

    #[test]
    fn lex_header() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);

        lexer.lex("# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6");

        assert_eq!(
            mock.tokens,
            vec![
                "h1",
                "word(H1)",
                "line_break",
                "h2",
                "word(H2)",
                "line_break",
                "h3",
                "word(H3)",
                "line_break",
                "h4",
                "word(H4)",
                "line_break",
                "h5",
                "word(H5)",
                "line_break",
                "h6",
                "word(H6)",
                "line_break"
            ]
        );
    }

    #[test]
    fn lex_bold() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("regular **bold** word");
        lexer.lex("and __another__ bold word");
        lexer.lex("**bold with spaces**");

        assert_eq!(
            mock.tokens,
            vec![
                "word(regular)",
                "begin_bold",
                "word(bold)",
                "end_bold",
                "word(word)",
                "line_break",
                "word(and)",
                "begin_bold",
                "word(another)",
                "end_bold",
                "word(bold)",
                "word(word)",
                "line_break",
                "begin_bold",
                "word(bold)",
                "word(with)",
                "word(spaces)",
                "end_bold",
                "line_break"
            ]
        );
    }

    #[test]
    fn lex_italic() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("regular *italic* word");
        lexer.lex("and _italic_ bold word");
        lexer.lex("*italic with spaces*");

        assert_eq!(
            mock.tokens,
            vec![
                "word(regular)",
                "begin_italic",
                "word(italic)",
                "end_italic",
                "word(word)",
                "line_break",
                "word(and)",
                "begin_italic",
                "word(italic)",
                "end_italic",
                "word(bold)",
                "word(word)",
                "line_break",
                "begin_italic",
                "word(italic)",
                "word(with)",
                "word(spaces)",
                "end_italic",
                "line_break"
            ]
        );
    }

    #[test]
    fn lex_link() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("a regular [Link](https://a.com)");
        lexer.lex("and [Another Link](https://b.com) with spaces");

        assert_eq!(
            mock.tokens,
            vec![
                "word(a)",
                "word(regular)",
                "begin_label",
                "word(Link)",
                "end_label",
                "url(https://a.com)",
                "line_break",
                "word(and)",
                "begin_label",
                "word(Another)",
                "word(Link)",
                "end_label",
                "url(https://b.com)",
                "word(with)",
                "word(spaces)",
                "line_break"
            ]
        );
    }
}
