use chrono::DateTime;
use chrono_tz::Tz;

pub fn datetime(s: &DateTime<Tz>) -> ::askama::Result<String> {
    Ok(s.format(crate::DATE_TIME_FORMAT).to_string())
}

pub fn datetime_nice(s: &DateTime<Tz>) -> ::askama::Result<String> {
    Ok(s.format(crate::DATE_TIME_FORMAT_NICE).to_string())
}
