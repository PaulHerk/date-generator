use apd::random_date;
use clap::Parser;
use regex::Regex;
use std::{fs, vec::IntoIter};
use time::OffsetDateTime;
mod parsers;
use parsers::*;

/// Simple program to append dates to your sources
#[derive(Parser, Debug)]
#[command(author, version, author, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long)]
    input: String,

    /// Name of the output file
    #[arg(default_value_t = format!("sources.txt"))]
    output: String,

    /// The date when to start;
    /// Format: YYYY-MM-DD_HH,
    /// Or: today + N, example: -e -2 is yesterday
    #[arg(short, long, allow_hyphen_values = true, value_parser = date_parser)]
    start_date: OffsetDateTime,

    /// The date when to end;
    /// Format is the same as for START_DATE
    #[arg(short, long, default_value = "0", allow_hyphen_values = true, value_parser = date_parser)]
    end_date: OffsetDateTime,

    /// The range in the day from when to pick values "HH-HH" (not inclusive)
    #[arg(short, long, value_parser = day_range_parser,  default_value = "9-20")]
    day_range: IntoIter<u8>,

    /// Output formatting
    #[arg(short, long, default_value_t = format!("[day].[month].[year] [hour]:[minute]"))]
    format: String,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&//=]*)").unwrap();

    let path = args.input;
    let file_content = fs::read_to_string(path).unwrap().trim().to_string();

    let mut new_file_content = String::new();
    for cap in regex.find_iter(&file_content) {
        let rand_date = random_date(
            args.start_date,
            args.end_date,
            &args.format,
            args.day_range.clone(),
        );
        let insert_string = &format!("{} {}\n", cap.as_str(), rand_date);
        new_file_content.push_str(&insert_string);
    }

    fs::write(args.output, &new_file_content)
        .unwrap_or_else(|_| panic!("Failed to write file, output: \n {}", new_file_content));
}
