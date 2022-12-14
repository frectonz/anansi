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
            Some('#') => self.lex_header(line),
            Some('!') => self.lex_image(line),
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
        self.lex_inline_code(word)
            .or_else(|| self.lex_bold(word))
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
                self.lex_word(&word[2..word.len() - 2]);
                self.collector.end_bold();
            } else {
                self.lex_word(&word[2..]);
            }
            Some(())
        } else if word.ends_with("**") || word.ends_with("__") {
            self.lex_word(&word[..word.len() - 2]);
            self.collector.end_bold();
            Some(())
        } else if word.ends_with("**.") || word.ends_with("__.") {
            let word = format!("{}.", &word[..word.len() - 3]);
            self.lex_word(&word);
            self.collector.end_bold();
            Some(())
        } else {
            None
        }
    }

    fn lex_italic(&mut self, word: &str) -> Option<()> {
        if word.starts_with('*') || word.starts_with('_') {
            self.collector.begin_italic();
            if word.ends_with('*') || word.ends_with('_') {
                self.lex_word(&word[1..word.len() - 1]);
                self.collector.end_italic();
            } else {
                self.lex_word(&word[1..]);
            }
            Some(())
        } else if word.ends_with('*') || word.ends_with('_') {
            self.lex_word(&word[..word.len() - 1]);
            self.collector.end_italic();
            Some(())
        } else if word.ends_with("*.") || word.ends_with("_.") {
            let word = format!("{}.", &word[..word.len() - 2]);
            self.lex_word(&word);
            self.collector.end_italic();
            Some(())
        } else {
            None
        }
    }

    fn lex_inline_code(&mut self, word: &str) -> Option<()> {
        if let Some(word) = word.strip_prefix('`') {
            self.collector.begin_inline_code();
            if let Some(word) = word.strip_suffix('`') {
                self.collector.word(word);
                self.collector.end_inline_code();
            } else {
                self.collector.word(word);
            }
            Some(())
        } else if let Some(word) = word.strip_suffix('`') {
            self.collector.word(word);
            self.collector.end_inline_code();
            Some(())
        } else if let Some(word) = word.strip_suffix("`.") {
            let word = word.to_owned() + ".";
            self.collector.word(&word);
            self.collector.end_inline_code();
            Some(())
        } else {
            None
        }
    }

    fn lex_label(&mut self, word: &str) -> Option<()> {
        if let Some(word) = word.strip_prefix('[') {
            self.collector.begin_label();

            if word.contains("](") {
                let mut parts = word.split("](");
                let label = parts.next().unwrap_or_default();
                let url = parts.next().unwrap_or_default();

                self.lex_word(label);
                self.collector.end_label();

                if let Some(url) = url.strip_suffix(").") {
                    self.collector.url(url);
                    self.collector.word(".");
                } else if let Some(url) = url.strip_suffix(')') {
                    self.collector.url(url);
                }
            } else {
                self.lex_word(word);
            }

            Some(())
        } else if word.contains(']') {
            let mut parts = word.split(']');
            let label = parts.next().unwrap_or_default();
            let url = parts.next().unwrap_or_default();

            self.lex_word(label);
            self.collector.end_label();

            if url.ends_with(").") {
                self.collector.url(&url[1..url.len() - 3]);
                self.collector.word(".");
            } else if url.ends_with(')') {
                self.collector.url(&url[1..url.len() - 1]);
            }

            Some(())
        } else {
            None
        }
    }

    fn lex_image(&mut self, line: &str) {
        let line = &line[1..];
        self.collector.image();
        line.split_whitespace().for_each(|word| {
            self.lex_word(word);
        });
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
        lexer.lex("**bold**");
        lexer.lex("regular **bold** word");
        lexer.lex("and __another__ bold word");
        lexer.lex("a **bold with spaces**.");

        assert_eq!(
            mock.tokens,
            vec![
                "begin_bold",
                "word(bold)",
                "end_bold",
                "line_break",
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
                "word(a)",
                "begin_bold",
                "word(bold)",
                "word(with)",
                "word(spaces.)",
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
        lexer.lex("a *italic with spaces*.");

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
                "word(a)",
                "begin_italic",
                "word(italic)",
                "word(with)",
                "word(spaces.)",
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
        lexer.lex("a [Link](https://a.com).");

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
                "line_break",
                "word(a)",
                "begin_label",
                "word(Link)",
                "end_label",
                "url(https://a.com)",
                "word(.)",
                "line_break"
            ]
        );
    }

    #[test]
    fn bold_link() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("**[Bold](https://a.com)**");

        assert_eq!(
            mock.tokens,
            vec![
                "begin_bold",
                "begin_label",
                "word(Bold)",
                "end_label",
                "url(https://a.com)",
                "end_bold",
                "line_break"
            ]
        );
    }

    #[test]
    fn bold_link_with_spaces() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("**[Bold Link](https://a.com)**");

        assert_eq!(
            mock.tokens,
            vec![
                "begin_bold",
                "begin_label",
                "word(Bold)",
                "word(Link)",
                "end_label",
                "url(https://a.com)",
                "end_bold",
                "line_break"
            ]
        );
    }

    #[test]
    fn italic_link() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("*[Italic](https://a.com)*");

        assert_eq!(
            mock.tokens,
            vec![
                "begin_italic",
                "begin_label",
                "word(Italic)",
                "end_label",
                "url(https://a.com)",
                "end_italic",
                "line_break"
            ]
        );
    }

    #[test]
    fn italic_link_with_spaces() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("*[Italic Link](https://a.com)*");

        assert_eq!(
            mock.tokens,
            vec![
                "begin_italic",
                "begin_label",
                "word(Italic)",
                "word(Link)",
                "end_label",
                "url(https://a.com)",
                "end_italic",
                "line_break"
            ]
        );
    }

    #[test]
    fn link_header() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("# [Header](https://a.com)");

        assert_eq!(
            mock.tokens,
            vec![
                "h1",
                "begin_label",
                "word(Header)",
                "end_label",
                "url(https://a.com)",
                "line_break"
            ]
        );
    }

    #[test]
    fn lex_image() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("![image](https://www.a.com)");

        assert_eq!(
            mock.tokens,
            vec![
                "img",
                "begin_label",
                "word(image)",
                "end_label",
                "url(https://www.a.com)",
                "line_break"
            ]
        );
    }

    #[test]
    fn lex_inline_code() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("regular `code` word");
        lexer.lex("a `code with spaces`.");

        assert_eq!(
            mock.tokens,
            vec![
                "word(regular)",
                "begin_inline_code",
                "word(code)",
                "end_inline_code",
                "word(word)",
                "line_break",
                "word(a)",
                "begin_inline_code",
                "word(code)",
                "word(with)",
                "word(spaces.)",
                "end_inline_code",
                "line_break"
            ]
        );
    }

    #[test]
    fn lex_inline_bold() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("an `inline **bold**`");

        assert_eq!(
            mock.tokens.join(" "),
            vec![
                "word(an)",
                "begin_inline_code",
                "word(inline)",
                "word(**bold**)",
                "end_inline_code",
                "line_break"
            ]
            .join(" ")
        );
    }

    #[test]
    fn bold_italic() {
        let mut mock = MockTokenCollector::default();
        let mut lexer = Lexer::new(&mut mock);
        lexer.lex("***bold italic***");

        assert_eq!(
            mock.tokens,
            vec![
                "begin_bold",
                "begin_italic",
                "word(bold)",
                "word(italic)",
                "end_italic",
                "end_bold",
                "line_break"
            ]
        );
    }
}
