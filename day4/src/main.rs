use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

static FIELDS: &'static [&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
// static FIELDS: &'static [&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = lines_from_file(filename);

    println!("part one result: {:?}", count_valid_passport(lines.clone()));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn count_valid_passport(lines: Vec<String>) -> u64 {
    let mut foo: u64 = 0;
    let mut count: u64 = 0;
    let mut field_keys: Vec<String> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        println!("line {:?}", line);
        if !line.is_empty() {
            let fields: Vec<&str> = line.split(' ').collect();
            let mut current_field_keys: Vec<String> = fields
                .into_iter()
                .map(|l| l.split(':').collect::<Vec<&str>>()[0].to_string())
                .collect::<Vec<String>>();

            field_keys.append(&mut current_field_keys);

            if (i != lines.len() - 1) {
                continue;
            }
        }
        foo += 1;

        println!("passport {:?}", field_keys);

        let mut fields: Vec<&str> = FIELDS.to_vec();

        println!("fields {:?}", fields);

        for field_key in field_keys.clone() {
            if let Some(pos) = fields.iter().position(|x| *x == field_key) {
                fields.remove(pos);
            }
        }

        println!("final fields {:?}", fields);

        if fields.is_empty() {
            count += 1
        }

        field_keys = Vec::new();

        // if field_keys.iter().any(|&field| field == "-i") {
        //     count += 1;
        // }
    }
    println!("foo {:?}", foo);

    count
}
