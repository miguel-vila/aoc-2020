use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, { // What's this?
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
pub fn read_numbers<P: AsRef<Path>>(filename: P) -> io::Result<Vec<i32>> {
    let mut vec = Vec::new();
    let lines = read_lines(filename)?;
    for line_it in lines {
        let line = line_it?;
        let n: i32 = line.parse().unwrap(); // unsafe, I assume
        vec.push(n);
    }
    Ok(vec)
}

