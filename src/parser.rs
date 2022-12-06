use crate::{Builder, Document, HeaderLevel, TokenCollector};

struct Transition {
    from: State,
    on: Event,
    to: State,
    action: Action,
}

impl From<(State, Event, State, Action)> for Transition {
    fn from((from, on, to, action): (State, Event, State, Action)) -> Self {
        Self {
            from,
            on,
            to,
            action,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Start,
    Header,
    Text,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Header,
    Text,
    StartBold,
    EndBold,
    StartItalic,
    EndItalic,
    StartLabel,
    EndLabel,
}

type Action = fn(&mut Builder);

pub struct Parser<'a> {
    state: State,
    transitions: Vec<Transition>,
    builder: &'a mut Builder,
}

impl<'a> TokenCollector for Parser<'a> {
    fn h1(&mut self) {
        self.handle_event(Event::Header);
        self.builder.set_header_level(HeaderLevel::H1);
    }

    fn h2(&mut self) {
        self.handle_event(Event::Header);
        self.builder.set_header_level(HeaderLevel::H2);
    }

    fn h3(&mut self) {
        self.handle_event(Event::Header);
        self.builder.set_header_level(HeaderLevel::H3);
    }

    fn h4(&mut self) {
        self.handle_event(Event::Header);
        self.builder.set_header_level(HeaderLevel::H4);
    }

    fn h5(&mut self) {
        self.handle_event(Event::Header);
        self.builder.set_header_level(HeaderLevel::H5);
    }

    fn h6(&mut self) {
        self.handle_event(Event::Header);
        self.builder.set_header_level(HeaderLevel::H6);
    }

    fn begin_bold(&mut self) {
        self.handle_event(Event::StartBold);
    }

    fn end_bold(&mut self) {
        self.handle_event(Event::EndBold);
    }

    fn begin_italic(&mut self) {
        self.handle_event(Event::StartItalic);
    }

    fn end_italic(&mut self) {
        self.handle_event(Event::EndItalic);
    }

    fn begin_label(&mut self) {
        self.handle_event(Event::StartLabel);
    }

    fn end_label(&mut self) {
        self.handle_event(Event::EndLabel);
    }

    fn url(&mut self, url: &str) {
        self.builder.add_url(url);
    }

    fn word(&mut self, word: &str) {
        self.builder.add_word(word);
    }

    fn line_break(&mut self) {
        self.builder.end_line();
    }
}

impl<'a> Parser<'a> {
    pub fn new(builder: &'a mut Builder) -> Self {
        let transitions = vec![
            // start transitions
            (
                State::Start,
                Event::Header,
                State::Header,
                (|b: &mut Builder| b.add_header()) as Action,
            )
                .into(),
            (
                State::Start,
                Event::Text,
                State::Text,
                (|b: &mut Builder| b.add_text()) as Action,
            )
                .into(),
            (
                State::Start,
                Event::StartBold,
                State::Text,
                (|b: &mut Builder| {
                    b.add_text();
                    b.start_bold()
                }) as Action,
            )
                .into(),
            // header transitions
            (
                State::Header,
                Event::Text,
                State::Text,
                (|b: &mut Builder| b.add_text()) as Action,
            )
                .into(),
            (
                State::Header,
                Event::StartLabel,
                State::Text,
                (|b: &mut Builder| b.start_label()) as Action,
            )
                .into(),
            // text transitions
            (
                State::Text,
                Event::Text,
                State::Text,
                (|b: &mut Builder| b.add_text()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::StartBold,
                State::Text,
                (|b: &mut Builder| b.start_bold()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::EndBold,
                State::Text,
                (|b: &mut Builder| b.end_bold()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::StartItalic,
                State::Text,
                (|b: &mut Builder| b.start_italic()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::EndItalic,
                State::Text,
                (|b: &mut Builder| b.end_italic()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::StartLabel,
                State::Text,
                (|b: &mut Builder| b.start_label()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::EndLabel,
                State::Text,
                (|b: &mut Builder| b.end_label()) as Action,
            )
                .into(),
        ];

        Self {
            transitions,
            state: State::Start,
            builder,
        }
    }

    pub fn parse(&mut self) -> Document {
        self.builder.get_document()
    }

    pub fn handle_event(&mut self, event: Event) {
        let transition = self
            .transitions
            .iter()
            .find(|t| t.from == self.state && t.on == event)
            .expect("No transition found");

        self.state = transition.to.clone();
        (transition.action)(self.builder);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_header() {}

    #[test]
    fn parse_bold() {}

    #[test]
    fn parse_italic() {}

    #[test]
    fn parse_link() {}

    #[test]
    #[ignore]
    fn parse_bold_link() {}
}
