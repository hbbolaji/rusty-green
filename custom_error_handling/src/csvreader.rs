use std::{
    error::Error,
    fmt::Display,
    fs::OpenOptions,
    io::{self, BufRead, BufReader},
    path::Path,
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
    FileNonExistent,
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

impl Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CsvError {}

pub fn read_csv<T: Copy + Default + FromStr>(filename: &str) -> Result<CsvData<T>> {
    let lines = read_to_lines(filename)?;

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
                res.map_err(|_| CsvError::CouldNotParseLine(e.into()))
            })
            .collect();

        let entries: Vec<T> = entries.into_iter().collect::<Result<_>>()?;
        if entries.len() == header.len() {
            data.push(entries);
        } else if entries.len() > header.len() {
            return Err(CsvError::LineTooLong(CsvLineLen {
                line_num: i,
                num_entries: entries.len(),
            }));
        } else {
            return Err(CsvError::LineTooShort(CsvLineLen {
                line_num: i,
                num_entries: entries.len(),
            }));
        }
    }

    return Ok(CsvData { header, data });
}

fn read_to_lines(filename: &str) -> Result<Vec<String>> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(CsvError::FileNonExistent);
    }

    let file = OpenOptions::new().read(true).open(path)?;
    let lines: Vec<_> = BufReader::new(file).lines().collect();
    lines
        .into_iter()
        .map(|line| line.map_err(|e| CsvError::CouldNotParseLine(Box::new(e))))
        .collect()

    // match file_res {
    //     Ok(file) => {
    //         let lines: Vec<_> = BufReader::new(file).lines().collect();
    //         lines
    //             .into_iter()
    //             .map(|line| line.map_err(|e| CsvError::CouldNotParseLine(Box::new(e))))
    //             .collect()
    //     }
    //     Err(e) => Err(CsvError::CouldNotOpenFile(e)),
    // }
}
