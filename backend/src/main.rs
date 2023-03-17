use regex::Regex;
use std::env::args;
use std::fs;

fn main() {
    let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&//=]*)").unwrap();
    let path = args().nth(1).expect("ski tissue");
    println!("Reading {}", path);

    let file_content = fs::read_to_string(path).unwrap().trim().to_string();
    dbg!(regex.is_match(&file_content));
    dbg!(&file_content);
    for cap in regex.captures_iter(&file_content) {
        dbg!(cap);
    }
}
