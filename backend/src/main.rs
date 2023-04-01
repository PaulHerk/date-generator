use apd::random_date;
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

    /// Output formatting ([day]-[month]-[year] [hour])
    #[arg(short, long, default_value_t = format!("[day].[month].[year] [hour]:00"))]
    out_format: String,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&//=]*)").unwrap();

    let path = args.input;
    let file_content = fs::read_to_string(path).unwrap().trim().to_string();

    let mut new_file_content = file_content.clone();
    for cap in regex.find_iter(&file_content) {
        let rand_date = random_date(&args.start_date, &args.end_date, &args.out_format);
        let insert_string = &format!(" {}", rand_date);
        new_file_content.insert_str(cap.end(), insert_string)
    }

    fs::write(args.output, &new_file_content)
        .unwrap_or_else(|_| panic!("Failed to write file, output: \n {}", new_file_content));
}
