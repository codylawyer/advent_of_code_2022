use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_1_read(filename: &str) -> Vec<i32> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Initialize vector
    let mut output = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut current_elf_calories = 0;
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        if !line.is_empty() {
            current_elf_calories += line.parse::<i32>().unwrap()
        } else {
            output.push(current_elf_calories);
            current_elf_calories = 0;
        }
    }
    output.push(current_elf_calories);

    // return the list of ints
    output
}

pub fn day_2_read(filename: &str) -> (String, String) {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Initialize the strings
    let mut opponent = String::new();
    let mut mine = String::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        if !line.is_empty() {
            opponent.push_str(&line[0..1]);
            mine.push_str(&line[2..3]);
        }
    }

    // return the strings
    (mine, opponent)
}
