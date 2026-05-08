pub mod csvreader;
use crate::csvreader::read_csv;
fn main() {
    let csv_data: csvreader::CsvData<i32> = read_csv("input.csv");
    println!("{:?}", csv_data);
}
