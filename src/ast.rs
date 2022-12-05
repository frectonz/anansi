#[derive(Debug, PartialEq)]
pub enum Token {
    Bold(Vec<Token>),
    Italic(Vec<Token>),
    Regular(String),
    Link { label: Vec<Token>, url: String },
}

#[derive(Debug, PartialEq)]
pub enum Line {
    Header {
        level: HeaderLevel,
        tokens: Vec<Token>,
    },
    Text(Vec<Token>),
}

#[derive(Debug, PartialEq)]
pub enum HeaderLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}
