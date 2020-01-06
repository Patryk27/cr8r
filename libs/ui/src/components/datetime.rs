use std::fmt;

pub type ChronoDateTime = chrono::DateTime<chrono::Utc>;

pub struct DateTime {
    datetime: ChronoDateTime,
}

impl DateTime {
    pub fn new(datetime: ChronoDateTime) -> Self {
        Self { datetime }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.datetime.format("%Y-%m-%d %H:%M:%S"))
    }
}
