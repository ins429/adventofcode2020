use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

static FIELDS: &'static [&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = lines_from_file(filename);

    println!("result: {:?}", count_valid_passport(lines.clone()));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn count_valid_passport(lines: Vec<String>) -> u64 {
    let mut count: u64 = 0;
    let mut field_keys: Vec<String> = Vec::new();
    let mut collected_fields: Vec<&str> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            let mut fields: Vec<&str> = line.split(' ').collect();
            let mut current_field_keys: Vec<String> = fields
                .clone()
                .into_iter()
                .map(|l| l.split(':').collect::<Vec<&str>>()[0].to_string())
                .collect::<Vec<String>>();

            field_keys.append(&mut current_field_keys);
            collected_fields.append(&mut fields);

            if i != lines.len() - 1 {
                continue;
            }
        }

        let mut fields: Vec<&str> = FIELDS.to_vec();

        for field_key in field_keys.clone() {
            if let Some(pos) = fields.iter().position(|x| *x == field_key) {
                fields.remove(pos);
            }
        }

        if fields.is_empty() && validate(collected_fields) {
            count += 1
        }

        field_keys = Vec::new();
        collected_fields = Vec::new();
    }

    count
}

fn validate(fields: Vec<&str>) -> bool {
    let mut is_valid: bool = true;

    for field in fields {
        let split_field: Vec<&str> = field.split(':').collect();

        match split_field[0] {
            "byr" => validate_byr(split_field[1], &mut is_valid),
            "iyr" => validate_iyr(split_field[1], &mut is_valid),
            "eyr" => validate_eyr(split_field[1], &mut is_valid),
            "hgt" => validate_hgt(split_field[1], &mut is_valid),
            "hcl" => validate_hcl(split_field[1], &mut is_valid),
            "ecl" => validate_ecl(split_field[1], &mut is_valid),
            "pid" => validate_pid(split_field[1], &mut is_valid),
            _ => (),
        }

        if !is_valid {
            return is_valid;
        }
    }

    is_valid
}

fn validate_byr(byr: &str, is_valid: &mut bool) -> () {
    let byr = byr.parse::<u32>().unwrap();

    if byr >= 1920 && byr <= 2002 {
        *is_valid = true
    } else {
        *is_valid = false
    }
}

fn validate_iyr(iyr: &str, is_valid: &mut bool) -> () {
    let iyr = iyr.parse::<u32>().unwrap();

    if iyr >= 2010 && iyr <= 2020 {
        *is_valid = true
    } else {
        *is_valid = false
    }
}

fn validate_eyr(eyr: &str, is_valid: &mut bool) -> () {
    let eyr = eyr.parse::<u32>().unwrap();

    if eyr >= 2020 && eyr <= 2030 {
        *is_valid = true
    } else {
        *is_valid = false
    }
}

fn validate_hgt(hgt: &str, is_valid: &mut bool) -> () {
    *is_valid = match Regex::new(r"^(\d+)(cm|in)$").unwrap().captures(&hgt) {
        None => false,
        Some(captures) => {
            let value = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
            match captures.get(2).unwrap().as_str() {
                "cm" => value >= 150 && value <= 193,
                "in" => value >= 59 && value <= 76,
                _ => false,
            }
        }
    }
}

fn validate_hcl(hcl: &str, is_valid: &mut bool) -> () {
    *is_valid = Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(hcl)
}

fn validate_ecl(ecl: &str, is_valid: &mut bool) -> () {
    *is_valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
}

fn validate_pid(pid: &str, is_valid: &mut bool) -> () {
    *is_valid = Regex::new(r"^[0-9]{9}$").unwrap().is_match(pid)
}
