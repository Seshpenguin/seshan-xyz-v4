use chrono::{DateTime, Utc, Datelike};

pub fn get_date_string() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_year() -> u32 {
    let now: DateTime<Utc> = Utc::now();
    now.year() as u32
}