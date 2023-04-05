#![allow(dead_code)]
use std::{fmt, ops::RangeInclusive};
use time::{format_description, format_description::FormatItem, Duration, OffsetDateTime};

#[derive(Debug, Clone)]
pub struct RangeInclusiveu8(pub RangeInclusive<u8>);

impl RangeInclusiveu8 {
    pub fn from(range: RangeInclusive<u8>) -> Self {
        Self(range)
    }
}

impl fmt::Display for RangeInclusiveu8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0.start(), self.0.end())
    }
}

pub fn day_range_parser(input: &str) -> Result<RangeInclusiveu8, String> {
    let (start, end) = input.split_once('-').expect("Wrong Syntax, \"hh-hh\".");
    let [start, end] = [start, end].map(|num| num.parse::<u8>().expect("Expected a number."));

    if start >= 24 {
        return Err("Invalid hour.".to_string());
    } else if start > end {
        return Err("End must be bigger than start.".to_string());
    } else if start == end {
        return Ok(RangeInclusiveu8::from(2..=1));
    }

    Ok(RangeInclusiveu8::from(start..=end))
}

pub fn date_parser(date: &str) -> Result<OffsetDateTime, String> {
    if let Ok(date) = date.parse::<i64>() {
        let today = OffsetDateTime::now_utc();
        let date = today + Duration::days(date);
        return Ok(date);
    }

    let format = format_description::parse(
        "[year]-[month]-[day]_[hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .unwrap();
    let error_msg =
        |_| format!("The inputted date is wrong, it must be \"YYYY-MM-DD_HH\", you have: {date}");
    OffsetDateTime::parse(&format!("{date}:00:00 +00:00:00"), &format).map_err(error_msg)
}

pub fn format_parser(format: &str) -> Result<Vec<FormatItem>, String> {
    let format_description = format_description::parse(format);
    let error_msg = |error| {
        format!(
            "The formatting is wrong, please check the documentation https://time-rs.github.io/book/api/format-description.html \n Complete error message: {error}"
        )
    };

    format_description.map_err(error_msg)
}
