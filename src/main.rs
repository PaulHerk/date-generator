use apd::random_date;
use clap::Parser;
use std::{fs, vec::IntoIter};
use time::OffsetDateTime;
mod parsers;
use linkify::LinkFinder;
use parsers::*;

/// Simple program to append dates to your sources
#[derive(Parser, Debug)]
#[command(author, version, author, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long, default_value = None)]
    input: Option<String>,

    /// Name of the output file
    #[arg(short, long, default_value = None)]
    output: Option<String>,

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

    let file_content = if let Some(path) = args.input {
        fs::read_to_string(path).unwrap().trim().to_string()
    } else {
        let mut buffer = String::new();
        let stdin = std::io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        buffer
    };

    let linkfinder = LinkFinder::new();
    let links = linkfinder.links(&file_content);

    let mut new_file_content = file_content.clone();
    let mut offset = 0_usize;
    for cap in links {
        let rand_date = random_date(
            args.start_date,
            args.end_date,
            &args.format,
            args.day_range.clone(),
        );
        let insert_string = &format!(" {}", rand_date);

        new_file_content.insert_str(offset + cap.end(), insert_string);
        offset += insert_string.len();
    }

    if let Some(output_path) = args.output {
        fs::write(output_path, &new_file_content)
            .unwrap_or_else(|_| panic!("Failed to write file, output: \n {}", new_file_content));
    } else {
        println!("---------- OUTPUT ----------");
        println!("{}", new_file_content);
    }
}
