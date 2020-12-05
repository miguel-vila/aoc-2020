use std::io;
mod file_reading;
use file_reading::read_lines;

#[derive(Debug)]
struct Rule {
    character: char,
    n1: i32,
    n2: i32
}

impl Rule {
    fn correct1(&self, passwd: &String) -> bool {
        let mut count = 0;
        for c in passwd.chars() {
            if c == self.character {
                count+=1;
            }
        }
        count >= self.n1 && count <= self.n2
    }
    fn correct2(&self, passwd: &String) -> bool {
        let chrs = passwd.as_bytes();
        let first  = chrs[(self.n1-1) as usize] as char == self.character;
        let second = chrs[(self.n2-1) as usize] as char == self.character;
        (first && !second) || (!first && second) 
    }
}

fn read_rules_and_passwords() -> io::Result<Vec<(Rule, String)>> {
    let lines = read_lines("./input2.txt")?;
    let mut vec = Vec::new();
    for line_it in lines {
        let line = line_it?;
        let spl: Vec<&str> = line.split(" ").collect();
        let nums: Vec<&str>  = spl[0].split("-").collect();
        let n1: i32 = nums[0].parse().unwrap();
        let n2: i32 = nums[1].parse().unwrap();
        let character = spl[1].chars().next().unwrap();
        let rule = Rule { character: character, n1: n1, n2: n2};
        let passwd = spl[2].to_owned(); // ???
        vec.push((rule, passwd));
    }
    Ok(vec)
}

fn main() {
    if let Ok(input) = read_rules_and_passwords() {
        let mut count1 = 0;
        let mut count2 = 0;
        for (rule, passwd) in input {
            if rule.correct1(&passwd) {
                count1 += 1;
            }
            if rule.correct2(&passwd) {
                count2 += 1;
            }
        }
        println!("Part 1: {}", count1);
        println!("Part 2: {}", count2);
    }
}