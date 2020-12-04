use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = lines_from_file(filename);

    println!(
        "part one result: {:?}",
        count_tree_encounter(lines.clone(), 3, 1)
    );

    println!(
        "part two result: {:?}",
        count_tree_encounter(lines.clone(), 1, 1)
            * count_tree_encounter(lines.clone(), 3, 1)
            * count_tree_encounter(lines.clone(), 5, 1)
            * count_tree_encounter(lines.clone(), 7, 1)
            * count_tree_encounter(lines, 1, 2)
    );
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn count_tree_encounter(lines: Vec<String>, right: u8, down: u8) -> u64 {
    let mut count: u64 = 0;
    let mut index: usize = 0;
    let mut down_count: u8 = down - 1;

    for line in lines {
        if down != 1 && down_count < down - 1 {
            down_count = down_count + 1;
            continue;
        }
        down_count = 0;
        let line_chars: Vec<char> = line.chars().collect();
        let target_pos: usize = index + right as usize;

        if line_chars[index] == '#' {
            count += 1;
        }

        if line_chars.len() <= target_pos {
            index = target_pos - line_chars.len()
        } else {
            index = target_pos
        };
    }

    count
}
