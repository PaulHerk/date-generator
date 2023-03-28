use rand::prelude::*;
use time::{format_description, OffsetDateTime};

pub fn random_date(start: &str, end: &str) -> OffsetDateTime {
    let format = format_description::parse(
        "[year]-[month]-[day]_[hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .unwrap();
    let start_date = OffsetDateTime::parse(&format!("{start} +00:00:00"), &format)
        .unwrap()
        .unix_timestamp();

    let end_date = if end == "today" {
        OffsetDateTime::parse(&format!("{end} +00:00:00"), &format)
            .unwrap()
            .unix_timestamp()
    } else {
        OffsetDateTime::now_utc().unix_timestamp()
    };

    let random_unix = rand::thread_rng().gen_range(start_date..=end_date);
    let random_date = OffsetDateTime::from_unix_timestamp(random_unix).unwrap();
    println!("{}", random_date);

    random_date
}

#[cfg(test)]
mod tests {
    use super::*;
}
