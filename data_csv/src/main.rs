use csv::{Reader, ReaderBuilder};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: String,
    name: String,
    email: String,
    created_at: String,
}

fn main() {
    test_regex();

    let file_name = "./data1.csv";
    let mut builder = ReaderBuilder::new();
    builder
        .double_quote(false)
        .comment(Some(b'-'))
        .has_headers(true);
    let result = builder.from_path(file_name);
    // let result: Result<Reader<std::fs::File>, csv::Error> = Reader::from_path(file_name);

    if result.is_err() {
        println!("Failed to read csv data");
        std::process::exit(9)
    }

    let mut my_reader = result.unwrap();
    println!("{:?}", my_reader.headers().unwrap());
    for record in my_reader.deserialize() {
        let person: User = record.unwrap();
        println!("your name is :{}", person.name);
    }
}

fn test_regex() {
    let my_pattern = "[A-Z]{1}[a-z]{2, 9}";
    let input_text = "Hashim is pulling a all nighter";
    let name_regex = Regex::new(my_pattern);
    if name_regex.is_err() {
        panic!("Invalid Regex Pattern")
    }

    let match_result = name_regex.unwrap().find(input_text);

    if let Some(my_match) = match_result {
        println!("{:?}", my_match.as_str())
    }
    println!("Did input patter match: {:?}", match_result);
}
