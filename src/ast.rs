#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Bold(Vec<Token>),
    Italic(Vec<Token>),
    InlineCode(Vec<Token>),
    Regular(String),
    Link { label: Vec<Token>, url: String },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    Header {
        level: HeaderLevel,
        tokens: Vec<Token>,
    },
    Paragraph(Vec<Token>),
    Image {
        label: Vec<Token>,
        url: String,
    },
    Blank,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HeaderLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

pub type Document = Vec<Line>;
