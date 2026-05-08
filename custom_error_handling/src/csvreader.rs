use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
pub struct CsvData<T: Copy + Default + FromStr> {
    pub header: Vec<String>,
    pub data: Vec<Vec<T>>,
}

pub fn read_csv<T: Copy + Default + FromStr>(filename: &str) -> CsvData<T> {
    let lines = read_to_lines(filename);

    if lines.is_empty() {
        return CsvData {
            header: vec![],
            data: vec![],
        };
    }

    let header: Vec<String> = lines[0].split(",").map(|s| s.into()).collect();
    let mut data: Vec<Vec<T>> = Vec::with_capacity(lines.len() - 1);

    for i in 1..lines.len() {
        let entries: Vec<T> = lines[i]
            .split(",")
            .map(|e| e.parse::<T>())
            .map(|e| e.unwrap_or_default())
            .collect();
        if entries.len() >= header.len() {
            data.push(entries[0..header.len()].into());
        }
    }

    return CsvData { header, data };
}

fn read_to_lines(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    if !path.exists() {
        return vec![];
    }

    let file_res = OpenOptions::new().read(true).open(path);
    match file_res {
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            lines.filter(|l| l.is_ok()).map(|l| l.unwrap()).collect()
        }
        Err(_) => vec![],
    }
}