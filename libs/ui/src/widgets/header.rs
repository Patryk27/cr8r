use std::fmt;

pub struct HeaderWidget<'a> {
    text: &'a str,
}

impl<'a> HeaderWidget<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl fmt::Display for HeaderWidget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        let header = format!("# {}", self.text)
            .blue()
            .bold();

        writeln!(f, "{}", header)
    }
}