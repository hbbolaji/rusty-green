

use csv::{Reader, ReaderBuilder};

fn main() {
  let file_name = "./data.csv";
  let mut builder = ReaderBuilder::new();
  builder.double_quote(false).comment(Some(b'-')).has_headers(true);
  let result = builder.from_path(file_name);
  // let result: Result<Reader<std::fs::File>, csv::Error> = Reader::from_path(file_name);

  if result.is_err() {
    println!("Failed to read csv data");
    std::process::exit(9)
  }

  
  let mut my_reader = result.unwrap();
  println!("{:?}", my_reader.headers().unwrap());
  for record in my_reader.records() {
    println!("your name is : {:?}", record.unwrap().get(2).unwrap())
  }
}
