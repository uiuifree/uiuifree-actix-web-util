use chrono::{DateTime, Local, TimeZone, Utc};
use mysql::Row;

pub mod connect;

pub fn mysql_now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn mysql_time_to_elastic(value: &str) -> Option<String> {
    let dt: Result<DateTime<Local>, _> = Local.datetime_from_str(value, "%Y-%m-%d %H:%M:%S");
    if dt.is_err() {
        return None;
    }
    let dt = dt.unwrap();
    return Some(dt.format("%Y-%m-%dT%H:%M:%S").to_string());
}

pub fn mysql_datetime_string_from_row(row: &Row, index: &str) -> Option<String> {
    let mysql_created_at = row.get(index);
    if let Some(..) = mysql_created_at {
        let mysql_created_at: Option<mysql_common::chrono::NaiveDateTime> =
            mysql_created_at.unwrap();
        if mysql_created_at.is_none() {
            return None;
        }
        return Some(mysql_created_at.unwrap().to_string());
    }
    None
}

pub fn timestamp_to_datetime(time: i64) -> String {
    Utc.timestamp(time, 0)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
