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

    /// Start date
    #[arg(short, long)]
    start_date: String,

    /// End date
    #[arg(short, long, default_value_t = ("yesterday".to_string()))]
    end_date: String,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&//=]*)").unwrap();

    let path = args.input;
    let file_content = fs::read_to_string(path).unwrap().trim().to_string();

    for cap in regex.captures_iter(&file_content) {
        dbg!(cap);
        let rand_date = random_date(&args.start_date, &args.end_date);
    }
}
