#[macro_use] extern crate maplit;

mod file_reading;
use file_reading::read_lines;
use std::collections::{ HashMap, HashSet };
use std::iter::Iterator;

fn parse_passports(lines: &Vec<String>) -> Vec<HashMap<String, String>> {
    let mut vec = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let mut passport = HashMap::new();
        while i < lines.len() && lines[i].len() > 0 {
            let line = lines[i].to_owned();
            let segments = line.split(" ");
            for segment in segments {
                let spl: Vec<&str> = segment.split(":").collect();
                passport.insert(spl[0].to_owned(), spl[1].to_owned());
            }
            i+=1;
        }
        vec.push(passport);
        i+=1;
    }
    vec
}

#[allow(dead_code)]
fn is_valid(passport: &HashMap<String, String>) -> bool {
    let keys: HashSet<&String> = passport.keys().collect();
    keys.len() == 8 || (keys.len() == 7 && ! passport.contains_key("cid"))
}

#[allow(dead_code)]
fn is_valid_2(passport: &HashMap<String, String>) -> bool {
    let valid_byr: bool = passport.get("byr").map(valid_byr).unwrap_or(false);
    let valid_iyr: bool = passport.get("iyr").map(valid_iyr).unwrap_or(false);
    let valid_eyr: bool = passport.get("eyr").map(valid_eyr).unwrap_or(false);
    let valid_hgt: bool = passport.get("hgt").map(valid_hgt).unwrap_or(false);
    let valid_ecl: bool = passport.get("ecl").map(valid_ecl).unwrap_or(false);
    let valid_pid: bool = passport.get("pid").map(valid_pid).unwrap_or(false);
    let valid_hcl: bool = passport.get("hcl").map(valid_hcl).unwrap_or(false);

    valid_byr &&
    valid_iyr &&
    valid_eyr &&
    valid_hgt &&
    valid_ecl &&
    valid_pid &&
    valid_hcl
}

fn validate_number(min: i32, max: i32, str: &String) -> bool  {
    let n: Result<i32,_> = str.parse();
    n.map(|n| min <= n && n <= max).unwrap_or(false)
}

fn valid_byr(str: &String) -> bool {
    validate_number(1920, 2002, str)
}

fn valid_iyr(str: &String) -> bool {
    validate_number(2010, 2020, str)
}

fn valid_eyr(str: &String) -> bool {
    validate_number(2020, 2030, str)
}

fn valid_hgt(str: &String) -> bool {
    if str.len() == 5 && str.chars().nth(3).unwrap() == 'c' && str.chars().nth(4).unwrap() == 'm' {
        validate_number(150, 193, &str[0..3].to_owned())
    } else if str.len() == 4 && str.chars().nth(2).unwrap() == 'i' && str.chars().nth(3).unwrap() == 'n' {
        validate_number(59, 76, &str[0..2].to_owned())
    } else {
        false
    }
}

fn valid_hcl(str: &String) -> bool {
    if str.len() != 7 || str.chars().nth(0).unwrap() != '#' {
        return false;
    }
    for c in str[1..7].chars() {
        if !c.is_digit(16) {
            return false
        }
    }
    true
}

fn valid_ecl(str: &String) -> bool {
    let valid = hashset!(
        "amb".to_owned(), 
        "blu".to_owned(), 
        "brn".to_owned(), 
        "gry".to_owned(), 
        "grn".to_owned(), 
        "hzl".to_owned(), 
        "oth".to_owned()
    );
    valid.contains(str)
}

fn valid_pid(str: &String) -> bool {
    if str.len() != 9 {
        return false
    }
    for c in str.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    return true
}

fn main() {
    if let Ok(input) = read_lines("./input4.txt").and_then(Iterator::collect) {
        let passports = parse_passports(&input);
        let mut count = 0;
        for passport in passports {
            if is_valid_2(&passport) {
                count += 1
            }
        }
        println!("{}", count);
        println!("{}", valid_pid(&"012345678".to_owned()));
    }
}