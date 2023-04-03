use apd::{random_date, RangeInclusiveu8};
use clap::Parser;
use regex::Regex;
use std::fs;

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

    /// Format: YYYY-MM-DD_HH
    /// Or: today + N, example: -e -2 is yesterday
    #[arg(short, long, allow_hyphen_values = true)]
    start_date: String,

    /// Same as START_DATE
    #[arg(short, long, default_value_t = format!("0"), allow_hyphen_values = true)]
    end_date: String,

    /// Change night hour definition
    #[arg(short, long, value_parser = apd::day_time_parser,default_value = "9-21",  default_value_t = RangeInclusiveu8::from(9..=20))]
    day: RangeInclusiveu8,

    /// Output formatting
    #[arg(short, long, default_value_t = format!("[day].[month].[year] [hour]:00"))]
    out_format: String,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&//=]*)").unwrap();

    let path = args.input;
    let file_content = fs::read_to_string(path).unwrap().trim().to_string();

    let mut new_file_content = file_content.clone();
    let mut position_offset: usize = 0;
    for cap in regex.find_iter(&file_content) {
        let rand_date = random_date(
            &args.start_date,
            &args.end_date,
            &args.out_format,
            &args.day.0,
        );
        let insert_string = &format!(" {}", rand_date);
        let position = position_offset + cap.end();
        new_file_content.insert_str(position, insert_string);
        position_offset += rand_date.chars().count() + 1;
    }

    fs::write(args.output, &new_file_content)
        .unwrap_or_else(|_| panic!("Failed to write file, output: \n {}", new_file_content));
}
