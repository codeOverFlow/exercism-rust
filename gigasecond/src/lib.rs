extern crate chrono;
use chrono::*;


pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    start_date.checked_add(Duration::seconds(i64::pow(10,9))).unwrap_or(start_date)
}
