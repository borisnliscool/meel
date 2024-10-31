use std::time::SystemTime;

use chrono::{DateTime, Utc};

pub fn system_time_to_iso_string(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = system_time.into();
    datetime.to_rfc3339()
}

pub fn iso_string_to_system_time(iso_string: &str) -> Result<SystemTime, chrono::ParseError> {
    let datetime = DateTime::parse_from_rfc3339(iso_string)?;
    Ok(datetime.into())
}