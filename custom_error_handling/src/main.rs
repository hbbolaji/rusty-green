use crate::csvreader::{CsvData, read_csv};
mod csvreader;
fn main() {
    let csv_data: CsvData<i32> = read_csv("input.csv").unwrap();
    println!("{:?}", csv_data);
}
