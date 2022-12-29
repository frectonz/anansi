use crate::{Builder, HeaderLevel};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser as MParser, Tag};

pub struct Parser<'a> {
    builder: &'a mut Builder,
}

impl<'a> Parser<'a> {
    pub fn new(builder: &'a mut Builder) -> Self {
        Self { builder }
    }

    pub fn parse(&mut self, markdown_input: &str) {
        let options = Options::all();
        let parser = MParser::new_ext(markdown_input, options);

        for event in parser {
            match event {
                Event::Start(Tag::Heading(level, _, _)) => {
                    let level = match level {
                        HeadingLevel::H1 => HeaderLevel::H1,
                        HeadingLevel::H2 => HeaderLevel::H2,
                        HeadingLevel::H3 => HeaderLevel::H3,
                        HeadingLevel::H4 => HeaderLevel::H4,
                        HeadingLevel::H5 => HeaderLevel::H5,
                        HeadingLevel::H6 => HeaderLevel::H6,
                    };

                    self.builder.add_header(level);
                }
                Event::End(Tag::Heading(_, _, _)) => {
                    self.builder.end_line();
                }

                Event::Start(Tag::Paragraph) => {
                    self.builder.start_paragraph();
                }
                Event::End(Tag::Paragraph) => {
                    self.builder.end_line();
                }

                Event::Start(Tag::Emphasis) => {
                    self.builder.start_italic();
                }
                Event::End(Tag::Emphasis) => {
                    self.builder.end_italic();
                }

                Event::Start(Tag::Strong) => {
                    self.builder.start_bold();
                }
                Event::End(Tag::Strong) => {
                    self.builder.end_bold();
                }

                Event::Text(text) => {
                    text.split_whitespace().for_each(|word| {
                        self.builder.add_word(word);
                    });
                }

                _ => {}
            }
        }
    }
}
