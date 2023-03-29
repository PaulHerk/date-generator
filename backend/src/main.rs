use backend::random_date;
use clap::Parser;
use regex::Regex;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long)]
    input: String,

    /// Format: YYYY-MM-DD_HH
    /// Or: today + N, example: -e -2 is yesterday
    /// Start date
    #[arg(short, long, allow_hyphen_values = true)]
    start_date: String,

    /// End date
    #[arg(short, long, default_value_t = ("0".to_string()), allow_hyphen_values = true)]
    // don't remove parentheses
    end_date: String,

    /// Output formatting ([day]-[month]-[year] [hour])
    #[arg(short, long, default_value_t = ("[day].[month].[year] [hour]:00".to_string()))]
    out_format: String,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&//=]*)").unwrap();

    let path = args.input;
    let file_content = fs::read_to_string(path).unwrap().trim().to_string();

    for cap in regex.captures_iter(&file_content) {
        dbg!(cap);
        let rand_date = random_date(&args.start_date, &args.end_date, &args.out_format);
    }
}
