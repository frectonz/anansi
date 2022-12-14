pub trait TokenCollector {
    fn h1(&mut self);
    fn h2(&mut self);
    fn h3(&mut self);
    fn h4(&mut self);
    fn h5(&mut self);
    fn h6(&mut self);

    fn begin_bold(&mut self);
    fn end_bold(&mut self);

    fn begin_italic(&mut self);
    fn end_italic(&mut self);

    fn begin_inline_code(&mut self);
    fn end_inline_code(&mut self);

    fn begin_label(&mut self);
    fn end_label(&mut self);

    fn url(&mut self, url: &str);
    fn word(&mut self, text: &str);

    fn image(&mut self);

    fn line_break(&mut self);
}

#[cfg(test)]
pub mod tests {
    use super::TokenCollector;

    #[derive(Debug, Default)]
    pub struct MockTokenCollector {
        pub tokens: Vec<String>,
    }

    impl TokenCollector for MockTokenCollector {
        fn h1(&mut self) {
            self.tokens.push("h1".to_string());
        }

        fn h2(&mut self) {
            self.tokens.push("h2".to_string());
        }

        fn h3(&mut self) {
            self.tokens.push("h3".to_string());
        }

        fn h4(&mut self) {
            self.tokens.push("h4".to_string());
        }

        fn h5(&mut self) {
            self.tokens.push("h5".to_string());
        }

        fn h6(&mut self) {
            self.tokens.push("h6".to_string());
        }

        fn begin_bold(&mut self) {
            self.tokens.push("begin_bold".to_string());
        }

        fn end_bold(&mut self) {
            self.tokens.push("end_bold".to_string());
        }

        fn begin_italic(&mut self) {
            self.tokens.push("begin_italic".to_string());
        }

        fn end_italic(&mut self) {
            self.tokens.push("end_italic".to_string());
        }

        fn begin_inline_code(&mut self) {
            self.tokens.push("begin_inline_code".to_string());
        }

        fn end_inline_code(&mut self) {
            self.tokens.push("end_inline_code".to_string())
        }

        fn begin_label(&mut self) {
            self.tokens.push("begin_label".to_string());
        }

        fn end_label(&mut self) {
            self.tokens.push("end_label".to_string());
        }

        fn url(&mut self, url: &str) {
            self.tokens.push(format!("url({})", url));
        }

        fn word(&mut self, text: &str) {
            self.tokens.push(format!("word({})", text));
        }

        fn image(&mut self) {
            self.tokens.push("img".to_string());
        }

        fn line_break(&mut self) {
            self.tokens.push("line_break".to_string());
        }
    }
}
