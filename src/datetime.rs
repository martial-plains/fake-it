use chrono::{DateTime, TimeZone, Utc};

pub fn get_random_date_in_month(year: i32, month: u32) -> DateTime<Utc> {
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day = days_in_month[month as usize - 1];
    if month == 2 && year % 4 == 0 {
        day += 1;
    }
    Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}
