use std::fmt;

pub struct Header<'a> {
    text: &'a str,
}

impl<'a> Header<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl fmt::Display for Header<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;

        let header = format!("# {}", self.text)
            .blue()
            .bold();

        writeln!(f, "{}", header)
    }
}