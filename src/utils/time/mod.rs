use chrono::{Datelike, TimeZone, Utc};

pub fn prev_day() -> (String, String) {
    // Count time
    let today = Utc::now();
    let _since = Utc
        .ymd(today.year(), today.month(), today.day())
        .and_hms(0, 0, 1);
    let since = format!("{}", _since.format("%Y-%m-%dT%H:%M:%SZ"));
    let _until = Utc
        .ymd(today.year(), today.month(), today.day())
        .and_hms(23, 59, 59);
    let until = format!("{}", _until.format("%Y-%m-%dT%H:%M:%SZ"));

    (since, until)
}
