use chrono::DateTime;

pub fn reformat_datetime(datetime: &str) -> String {
    DateTime::parse_from_rfc3339(datetime)
        .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|_| "invalid date".to_string())
}