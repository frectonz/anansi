use crate::{Document, HeaderLevel, Line, Token};

#[derive(Debug, Default)]
pub struct Builder {
    lines: Vec<Line>,
    parsing: Vec<Parsing>,
    bold_tokens: Vec<Token>,
    italic_tokens: Vec<Token>,
    label_tokens: Vec<Token>,
}

#[derive(Debug)]
enum Parsing {
    Bold,
    Italic,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_document(&self) -> Document {
        self.lines.clone()
    }

    pub(crate) fn add_header(&mut self, level: HeaderLevel) {
        self.lines.push(Line::Header {
            level,
            tokens: Vec::new(),
        });
    }

    pub(crate) fn add_text(&mut self) {
        self.lines.push(Line::Paragraph(Vec::new()));
    }

    pub(crate) fn start_bold(&mut self) {
        self.parsing.push(Parsing::Bold);
    }

    pub(crate) fn end_bold(&mut self) {
        let tokens: Vec<Token> = self.bold_tokens.drain(..).collect();

        let wrap_with_bold = |tokens: &mut Vec<Token>| {
            if let Some(t) = tokens.last_mut() {
                if let Token::Italic(_) = t {
                    *t = Token::Bold(vec![t.clone()]);
                }
            }
        };

        if tokens.is_empty() {
            match self.lines.last_mut() {
                Some(Line::Header { tokens, .. }) => wrap_with_bold(tokens),
                Some(Line::Paragraph(tokens, ..)) => wrap_with_bold(tokens),
                Some(Line::Image { label, .. }) => wrap_with_bold(label),
                Some(Line::Blank) => {}
                None => {}
            };
        } else {
            let bold = Token::Bold(tokens);
            match self.lines.last_mut() {
                Some(Line::Header { tokens, .. }) => tokens.push(bold),
                Some(Line::Paragraph(tokens, ..)) => tokens.push(bold),
                Some(Line::Image { label, .. }) => label.push(bold),
                Some(Line::Blank) => {}
                None => {}
            };
        }

        self.parsing.pop();
    }

    pub(crate) fn start_italic(&mut self) {
        self.parsing.push(Parsing::Italic);
    }

    pub(crate) fn end_italic(&mut self) {
        let italic = Token::Italic(self.italic_tokens.drain(..).collect());
        match self.lines.last_mut() {
            Some(Line::Header { tokens, .. }) => tokens.push(italic),
            Some(Line::Paragraph(tokens, ..)) => tokens.push(italic),
            Some(Line::Image { label, .. }) => label.push(italic),
            Some(Line::Blank) => {}
            None => {}
        };

        self.parsing.pop();
    }

    pub(crate) fn add_word(&mut self, word: &str) {
        match self.parsing.last() {
            Some(Parsing::Bold) => self.bold_tokens.push(Token::Regular(word.to_string())),
            Some(Parsing::Italic) => self.italic_tokens.push(Token::Regular(word.to_string())),
            None => match self.lines.last_mut() {
                Some(Line::Header { tokens, .. }) => tokens.push(Token::Regular(word.to_string())),
                Some(Line::Paragraph(tokens, ..)) => tokens.push(Token::Regular(word.to_string())),
                Some(Line::Image { .. }) => {}
                Some(Line::Blank) => {}
                None => {}
            },
        }
    }

    pub(crate) fn end_line(&mut self) {
        self.parsing.clear();
        self.bold_tokens.clear();
        self.italic_tokens.clear();
        self.label_tokens.clear();
    }
}
