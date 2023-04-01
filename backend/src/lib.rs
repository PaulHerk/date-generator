use rand::prelude::*;
use time::{format_description, Duration, OffsetDateTime};

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

pub fn random_date(start: &str, end: &str, format: &str) -> String {
    let start_date = get_date(start).unix_timestamp();
    let end_date = get_date(end).unix_timestamp();

    let random_unix = rand::thread_rng().gen_range(start_date..=end_date);
    let random_date = OffsetDateTime::from_unix_timestamp(random_unix).unwrap();

    let format = format_description::parse(format).unwrap();
    let random_date_formatted = random_date.format(&format).unwrap();
    println!("{}", random_date_formatted);
    random_date_formatted
}

#[cfg(test)]
mod tests {
    use super::*;
}
