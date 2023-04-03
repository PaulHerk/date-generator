use core::ops::RangeInclusive;
use rand::prelude::*;
use std::fmt;
use time::{format_description, Duration, OffsetDateTime};

#[derive(Debug, Clone)]
pub struct RangeInclusiveu8(pub RangeInclusive<u8>);

impl RangeInclusiveu8 {
    pub fn from(range: RangeInclusive<u8>) -> Self {
        Self { 0: range }
    }
}

impl fmt::Display for RangeInclusiveu8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0.start(), self.0.end())
    }
}

fn get_date(date: &str) -> OffsetDateTime {
    if let Ok(date) = date.parse::<i64>() {
        let today = OffsetDateTime::now_utc();
        let date = today + Duration::days(date);
        return date;
    }

    let format = format_description::parse(
        "[year]-[month]-[day]_[hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .unwrap();
    OffsetDateTime::parse(&format!("{date}:00:00 +00:00:00"), &format).unwrap()
}

pub fn random_date(start: &str, end: &str, format: &str, day_time: &RangeInclusive<u8>) -> String {
    let start_date = get_date(start).unix_timestamp();
    let end_date = get_date(end).unix_timestamp();

    let random_unix = rand::thread_rng().gen_range(start_date..=end_date);
    let mut random_date = OffsetDateTime::from_unix_timestamp(random_unix).unwrap();

    if !day_time.contains(&random_date.hour()) {
        let random_hour = rand::thread_rng().gen_range(day_time.clone());
        random_date = random_date.replace_hour(random_hour).unwrap();
    }

    let format = format_description::parse(format).unwrap();
    let random_date_formatted = random_date.format(&format).unwrap();
    random_date_formatted
}

pub fn day_time_parser(input: &str) -> Result<RangeInclusiveu8, String> {
    let (start, end) = input.split_once('-').expect("Wrong Syntax, \"hh-hh\".");
    let [start, end] = [start, end].map(|num| num.parse::<u8>().expect("Expected a number."));

    if start >= 24 {
        return Err("Invalid hour.".to_string());
    } else if start > end {
        return Err("End must be bigger than start.".to_string());
    } else if start == end {
        return Ok(RangeInclusiveu8::from(4..=1));
    }

    Ok(RangeInclusiveu8::from(start..=end))
}

#[cfg(test)]
mod tests {
    use super::*;
}
