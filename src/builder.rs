use crate::{Document, HeaderLevel, Line, Token};

#[derive(Debug, Default)]
pub struct Builder {
    lines: Vec<Line>,
    line: Option<Line>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_document(&self) -> Document {
        self.lines.clone()
    }

    pub(crate) fn add_header(&mut self, level: HeaderLevel) {
        println!("add_header: {:?}", level);
        self.line = Some(Line::Header {
            level,
            tokens: Vec::new(),
        });
    }

    pub(crate) fn start_paragraph(&mut self) {
        self.line = Some(Line::Paragraph(Vec::new()));
    }

    pub(crate) fn start_bold(&mut self) {
        match self.line {
            Some(Line::Header { ref mut tokens, .. }) => tokens.push(Token::Bold(Vec::new())),
            Some(Line::Paragraph(ref mut tokens, ..)) => tokens.push(Token::Bold(Vec::new())),
            None => {}
        };
    }

    pub(crate) fn end_bold(&mut self) {}

    pub(crate) fn start_italic(&mut self) {
        match self.line {
            Some(Line::Header { ref mut tokens, .. }) => tokens.push(Token::Italic(Vec::new())),
            Some(Line::Paragraph(ref mut tokens, ..)) => tokens.push(Token::Italic(Vec::new())),
            None => {}
        };
    }

    pub(crate) fn end_italic(&mut self) {}

    pub(crate) fn add_word(&mut self, word: &str) {
        let last_token = match self.line {
            Some(Line::Header { ref mut tokens, .. }) => tokens.last_mut(),
            Some(Line::Paragraph(ref mut tokens, ..)) => tokens.last_mut(),
            None => None,
        };

        use Token::*;
        match last_token {
            Some(Bold(ref mut words)) => words.push(Regular(word.to_string())),
            Some(Italic(ref mut words)) => words.push(Regular(word.to_string())),
            Some(InlineCode(ref mut words)) => words.push(Regular(word.to_string())),
            Some(Link { ref mut label, .. }) => label.push(Regular(word.to_string())),
            _ => match self.line {
                Some(Line::Header { ref mut tokens, .. }) => tokens.push(Regular(word.to_string())),
                Some(Line::Paragraph(ref mut tokens, ..)) => tokens.push(Regular(word.to_string())),
                None => {}
            },
        };
    }

    pub(crate) fn end_line(&mut self) {
        if let Some(line) = self.line.take() {
            self.lines.push(line);
        }
    }
}
