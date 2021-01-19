use chrono::{Duration, Local};

pub fn with_weeks(weeks: i64) -> String {
    let now = Local::now();
    let duration = Duration::weeks(weeks);
    let later = now + duration;
    later.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn with_timezone(s: &str) -> String {
    let tz = s.parse::<chrono_tz::Tz>().unwrap();
    let now = Local::now();
    now.with_timezone(&tz).to_string()
}
