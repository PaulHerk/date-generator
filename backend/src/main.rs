use clap::Parser;
use rand::prelude::*;
use regex::Regex;
use std::fs;
use time::{format_description, OffsetDateTime};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long)]
    input: String,

    /// Start date
    #[arg(short, long)]
    start_date: String,

    /// End date
    #[arg(short, long)]
    end_date: String,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&//=]*)").unwrap();

    let path = args.input;
    let file_content = fs::read_to_string(path).unwrap().trim().to_string();

    for cap in regex.captures_iter(&file_content) {
        dbg!(cap);
    }

    let format = format_description::parse(
        "[year]-[month]-[day]_[hour]:[minute]:[second] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .unwrap();
    let start_date = OffsetDateTime::parse(&format!("{} +00:00:00", args.start_date), &format)
        .unwrap()
        .unix_timestamp();
    let end_date = OffsetDateTime::parse(&format!("{} +00:00:00", args.end_date), &format)
        .unwrap()
        .unix_timestamp();

    let random_unix = rand::thread_rng().gen_range(start_date..=end_date);
    let random_date = OffsetDateTime::from_unix_timestamp(random_unix).unwrap();
    println!("{}", random_date)
}
