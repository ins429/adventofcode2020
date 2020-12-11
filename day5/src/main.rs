use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = lines_from_file(filename);

    println!("part one result: {:?}", get_max_id(lines.clone()));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_max_id(lines: Vec<String>) -> String {
    let mut max: u64 = 0;
    let mut pick: String = "".to_string();
    let mut index: usize = 0;

    for line in lines {
        let result: u64 = 0;

        if result > max {
            pick = line;
            max = result;
        };
    }

    pick
}
