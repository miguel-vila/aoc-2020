mod file_reading;
use file_reading::read_lines;
use std::iter::Iterator;

struct Slope {
    x: i32,
    y: i32,
}

fn count(input: &Vec<String>, slope: &Slope) -> i32 {
    let length = input[0].len() as i32;
    let mut count = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while (y as usize) < input.len() {
        let line = &input[y as usize];
        if line.as_bytes()[x as usize] as char == '#' {
            count += 1;
        }
        x = (x + slope.x) % length;
        y += slope.y;
    }
    count
}

fn main() {
    if let Ok(input) = read_lines("./input3.txt").and_then(Iterator::collect) {
        let mut product = 1;
        let slopes = vec![
            Slope { x: 1, y: 1 },
            Slope { x: 3, y: 1 },
            Slope { x: 5, y: 1 },
            Slope { x: 7, y: 1 },
            Slope { x: 1, y: 2 },
        ];
        for slope in slopes {
            product *= count(&input, &slope);
        }
        println!("{}", product)
    }
}
