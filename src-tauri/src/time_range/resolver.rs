use chrono::{DateTime, Duration, Utc};
use super::TimeRange;

pub fn get_start_date(range: TimeRange) -> DateTime<Utc> {
    let now = Utc::now();
    match range {
        TimeRange::Day => now - Duration::days(1),
        TimeRange::Week => now - Duration::weeks(1),
        TimeRange::Month => now - Duration::days(30),
        TimeRange::Year => now - Duration::days(365),
    }
}
