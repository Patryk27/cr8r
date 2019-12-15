use std::fmt;

pub struct DateTime<'a> {
    datetime: &'a str,
}

impl<'a> DateTime<'a> {
    pub fn new(datetime: &'a str) -> Self {
        Self { datetime }
    }
}

impl fmt::Display for DateTime<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = chrono::DateTime::parse_from_rfc3339(self.datetime)
            .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|_| "invalid datetime".to_string());

        write!(f, "{}", str)
    }
}
