mod parsers;
use parsers::format_parser;
use rand::prelude::*;
use std::ops::RangeInclusive;
use time::OffsetDateTime;

pub fn random_date(
    start: OffsetDateTime,
    end: OffsetDateTime,
    format: &str,
    day_range: &RangeInclusive<u8>,
) -> String {
    let random_unix = rand::thread_rng().gen_range(start.unix_timestamp()..=end.unix_timestamp());
    let mut random_date = OffsetDateTime::from_unix_timestamp(random_unix).unwrap();

    if !day_range.contains(&random_date.hour()) {
        let random_hour = rand::thread_rng().gen_range(day_range.clone());
        random_date = random_date.replace_hour(random_hour).unwrap();
    }

    let format = format_parser(format).unwrap();
    random_date.format(&format).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
