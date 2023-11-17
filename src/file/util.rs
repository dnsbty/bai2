use chrono::{NaiveDate, NaiveTime};
use std::str::FromStr;

pub fn parse_string(string: &str) -> String {
    string.trim().replace("/", "")
}

pub fn parse_currency(string: &str, default: &str) -> String {
    return match parse_string(string).as_str() {
        "" => default.to_string(),
        c => c.to_string(),
    };
}

pub fn parse_date(string: &str) -> Option<NaiveDate> {
    let date = parse_string(string);
    let maybe_date = NaiveDate::parse_from_str(&date, "%y%m%d");
    match maybe_date {
        Ok(d) => Some(d),
        Err(_) => None,
    }
}

pub fn parse_time(string: &str) -> Option<String> {
    match parse_string(string).as_str() {
        "" => None,
        "2400" => Some("end of day".to_string()),
        "9999" => Some("end of day".to_string()),
        time => match NaiveTime::parse_from_str(time, "%H%M") {
            Ok(t) => Some(t.to_string()),
            Err(_) => None,
        },
    }
}

pub fn parse_int<T: FromStr>(string: &str) -> Option<T> {
    let number = string
        .trim()
        .replace("/", "")
        .trim_start_matches('0')
        .parse::<T>();

    match number {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}
