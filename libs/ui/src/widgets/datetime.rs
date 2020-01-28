use std::fmt;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub struct DateTimeWidget {
    datetime: DateTime,
}

impl DateTimeWidget {
    pub fn new(datetime: DateTime) -> Self {
        Self { datetime }
    }
}

impl fmt::Display for DateTimeWidget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.datetime.format("%Y-%m-%d %H:%M:%S"))
    }
}
