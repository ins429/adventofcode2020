use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = lines_from_file(filename);
    let mut result: Vec<i16> = Vec::new();
    let mut part_two_result: Vec<i16> = Vec::new();
    let mut final_result: i64 = 1;
    let mut part_two_final_result: i64 = 1;

    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if lines[i] + lines[j] == 2020 {
                result.push(lines[i]);
                result.push(lines[j]);
            }

            for k in j + 1..lines.len() {
                if lines[i] + lines[j] + lines[k] == 2020 {
                    part_two_result.push(lines[i]);
                    part_two_result.push(lines[j]);
                    part_two_result.push(lines[k]);
                }
            }
        }
    }

    for item in result {
        final_result *= item as i64
    }

    for item in part_two_result {
        part_two_final_result *= item as i64
    }

    println!("part 1 result: {:?}", final_result);
    println!("part 2 result: {:?}", part_two_final_result);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i16> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<i16>()
                .expect("Could not parse line into an integer")
        })
        .collect()
}
