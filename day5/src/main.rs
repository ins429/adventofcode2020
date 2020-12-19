use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = lines_from_file(filename);

    println!("result: {:?}", get_highest_seat_id(lines));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_highest_seat_id(lines: Vec<String>) -> u16 {
    let mut highest_seat_id: u16 = 0;

    for line in lines {
        let mut row_guess: u16 = 0;
        let mut col_guess: u16 = 0;
        let row: &str = &line[..7];
        let col: &str = &line[7..10];

        if &row[..1] == "B" {
            row_guess = row_guess + 64
        }
        if &row[1..2] == "B" {
            row_guess = row_guess + 32
        }
        if &row[2..3] == "B" {
            row_guess = row_guess + 16
        }
        if &row[3..4] == "B" {
            row_guess = row_guess + 8
        }
        if &row[4..5] == "B" {
            row_guess = row_guess + 4
        }
        if &row[5..6] == "B" {
            row_guess = row_guess + 2
        }
        if &row[6..7] == "B" {
            row_guess = row_guess + 1
        }
        if &col[..1] == "R" {
            col_guess = col_guess + 4
        }
        if &col[1..2] == "R" {
            col_guess = col_guess + 2
        }
        if &col[2..3] == "R" {
            col_guess = col_guess + 1
        }

        let result: u16 = row_guess * 8 + col_guess;

        if result > highest_seat_id {
            highest_seat_id = result
        }
    }

    highest_seat_id
}
