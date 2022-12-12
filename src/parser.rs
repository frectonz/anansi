use crate::{Builder, HeaderLevel, TokenCollector};

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
    StartInlineCode,
    EndInlineCode,
    StartLabel,
    EndLabel,
    EndLine,
    Word,
    Image,
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

    fn begin_inline_code(&mut self) {
        self.handle_event(Event::StartInlineCode);
    }

    fn end_inline_code(&mut self) {
        self.handle_event(Event::EndInlineCode);
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
        self.handle_event(Event::Word);
        self.builder.add_word(word);
    }

    fn image(&mut self) {
        self.handle_event(Event::Image);
    }

    fn line_break(&mut self) {
        self.handle_event(Event::EndLine);
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
                (|b: &mut Builder| {
                    eprintln!("got text");
                    b.add_text()
                }) as Action,
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
            (
                State::Start,
                Event::StartItalic,
                State::Text,
                (|b: &mut Builder| {
                    b.add_text();
                    b.start_italic()
                }) as Action,
            )
                .into(),
            (
                State::Start,
                Event::StartInlineCode,
                State::Text,
                (|b: &mut Builder| {
                    b.add_text();
                    b.start_inline_code();
                }) as Action,
            )
                .into(),
            (
                State::Start,
                Event::Word,
                State::Text,
                (|b: &mut Builder| b.add_text()) as Action,
            )
                .into(),
            (
                State::Start,
                Event::Image,
                State::Text,
                (|b: &mut Builder| b.add_image()) as Action,
            )
                .into(),
            (
                State::Start,
                Event::EndLine,
                State::Start,
                (|b: &mut Builder| b.blank_line()) as Action,
            )
                .into(),
            // header transitions
            (
                State::Header,
                Event::EndLine,
                State::Start,
                (|b: &mut Builder| b.end_line()) as Action,
            )
                .into(),
            (
                State::Header,
                Event::Text,
                State::Text,
                (|_: &mut Builder| {}) as Action,
            )
                .into(),
            (
                State::Header,
                Event::StartLabel,
                State::Text,
                (|b: &mut Builder| b.start_label()) as Action,
            )
                .into(),
            (
                State::Header,
                Event::Word,
                State::Text,
                (|_: &mut Builder| {}) as Action,
            )
                .into(),
            // text transitions
            (
                State::Text,
                Event::Text,
                State::Text,
                (|_: &mut Builder| {}) as Action,
            )
                .into(),
            (
                State::Text,
                Event::Word,
                State::Text,
                (|_: &mut Builder| {}) as Action,
            )
                .into(),
            (
                State::Text,
                Event::Header,
                State::Header,
                (|b: &mut Builder| b.add_header()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::EndLine,
                State::Start,
                (|b: &mut Builder| b.end_line()) as Action,
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
                Event::StartInlineCode,
                State::Text,
                (|b: &mut Builder| b.start_inline_code()) as Action,
            )
                .into(),
            (
                State::Text,
                Event::EndInlineCode,
                State::Text,
                (|b: &mut Builder| b.end_inline_code()) as Action,
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

    pub fn handle_event(&mut self, event: Event) {
        dbg!(&self.state, &event);
        let transition = self
            .transitions
            .iter()
            .find(|t| t.from == self.state && t.on == event)
            .expect("No transition found");
        dbg!(&transition.to);
        eprintln!();

        self.state = transition.to.clone();
        (transition.action)(self.builder);
    }
}
