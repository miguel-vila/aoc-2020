use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub enum ReadingError {
    IOError(io::Error),
    ParsingError(ParseIntError),
}

#[allow(dead_code)]
pub fn read_numbers<P: AsRef<Path>>(filename: P) -> Result<Vec<i32>, ReadingError> {
    let mut vec = Vec::new();
    let lines = read_lines(filename).map_err(ReadingError::IOError)?;
    for line_it in lines {
        let line = line_it.map_err(ReadingError::IOError)?;
        let n: i32 = line.parse().map_err(ReadingError::ParsingError)?;
        vec.push(n);
    }
    Ok(vec)
}
