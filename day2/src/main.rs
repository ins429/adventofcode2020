use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = lines_from_file(filename);
    let mut result: i64 = 0;
    let mut part_two_result: i64 = 0;

    for line in lines {
        if is_correct_part_one(line.clone()) {
            result += 1;
        }

        if is_correct_part_two(line) {
            part_two_result += 1;
        }
    }

    println!("part one result: {:?}", result);
    println!("part two result: {:?}", part_two_result);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn is_correct_part_one(line: String) -> bool {
    let v: Vec<&str> = line.split(' ').collect();
    let min_max: Vec<&str> = v[0].split('-').collect();
    let min: i32 = min_max[0]
        .parse::<i32>()
        .expect("Could not parse line into an integer");
    let max: i32 = min_max[1]
        .parse::<i32>()
        .expect("Could not parse line into an integer");
    let letter_pattern: Vec<&str> = v[1].split(':').collect();
    let letter: &str = letter_pattern[0];
    let password = v[2];

    let mut count: i32 = 0;

    for pw_char in password.chars() {
        if pw_char == letter.chars().collect::<Vec<char>>()[0] {
            count += 1;
        }
    }

    if count >= min && count <= max {
        true
    } else {
        false
    }
}

fn is_correct_part_two(line: String) -> bool {
    let v: Vec<&str> = line.split(' ').collect();
    let min_max: Vec<&str> = v[0].split('-').collect();
    let min: usize = min_max[0]
        .parse::<usize>()
        .expect("Could not parse line into an integer");
    let max: usize = min_max[1]
        .parse::<usize>()
        .expect("Could not parse line into an integer");
    let letter_pattern: Vec<&str> = v[1].split(':').collect();
    let letter: char = letter_pattern[0].chars().collect::<Vec<char>>()[0];
    let password: Vec<char> = v[2].chars().collect();

    if (password[min - 1] == letter || password[max - 1] == letter)
        && !(password[min - 1] == letter && password[max - 1] == letter)
    {
        true
    } else {
        false
    }
}
