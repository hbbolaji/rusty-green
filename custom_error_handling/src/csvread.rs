use std::{
    error::Error,
    fs::OpenOptions,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

type Result<T> = std::result::Result<T, CsvError>;

#[derive(Debug)]
pub struct CsvData<T: Copy + Default + FromStr> {
    pub header: Vec<String>,
    pub data: Vec<Vec<T>>,
}

#[derive(Debug)]
pub struct CsvLineLen {
    pub line_num: usize,
    pub num_entries: usize,
}

#[derive(Debug)]
pub enum CsvError {
    FileNonExistant,
    CouldNotOpenFile(io::Error),
    CouldNotParseLine(Box<dyn Error>),
    FileIsEmpty,
    CouldNotParseValue(String),
    LineTooShort(CsvLineLen),
    LineTooLong(CsvLineLen),
}

impl From<io::Error> for CsvError {
    fn from(value: io::Error) -> Self {
        Self::CouldNotOpenFile(value)
    }
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CsvError {}

pub fn read_csv<T: Copy + Default + FromStr>(filename: &str) -> Result<CsvData<T>> {
    let lines = read_to_lines(filename)?;
    // ERROR 4: file was empty
    if lines.is_empty() {
        return Err(CsvError::FileIsEmpty);
    }

    let header: Vec<String> = lines[0].split(",").map(|s| s.into()).collect();
    let mut data: Vec<Vec<T>> = Vec::with_capacity(lines.len() - 1);

    for i in 1..lines.len() {
        let entries: Vec<Result<T>> = lines[i]
            .split(",")
            .map(|e| {
                let res = e.parse::<T>();
                res.map_err(|_| CsvError::CouldNotParseValue(e.into()))
            })
            // ERROR 5: could not parse from string.
            .collect();

        let entries: Vec<T> = entries.into_iter().collect::<Result<_>>()?;
        // ERROR 6: line was too short.
        if entries.len() == header.len() {
            // ERROR 7 (hidden): line was too long.
            data.push(entries);
        } else if entries.len() < header.len() {
            return Err(CsvError::LineTooShort(CsvLineLen {
                line_num: i,
                num_entries: entries.len(),
            }));
        } else {
            return Err(CsvError::LineTooLong(CsvLineLen {
                line_num: i,
                num_entries: entries.len(),
            }));
        }
    }

    Ok(CsvData { header, data })
}

fn read_to_lines(filename: &str) -> Result<Vec<String>> {
    let path = std::path::Path::new(filename);
    // ERROR 1: file could be non-existant.
    if !path.exists() {
        return Err(CsvError::FileNonExistant);
    }
    let file = OpenOptions::new().read(true).open(path)?;
    let lines: Vec<_> = BufReader::new(file).lines().collect();
    // ERROR 3: line could not be parsed.
    lines
        .into_iter()
        .map(|line| line.map_err(|e| CsvError::CouldNotParseLine(Box::new(e))))
        .collect()
}
