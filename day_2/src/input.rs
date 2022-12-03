use std::fs;

pub fn get_raw_input() -> Vec<(char, char)> {
    let content = fs::read_to_string("input.txt").unwrap();
    // let content = "A Y\nB X\nC Z";
    content
        .split("\n")
        .map(|row| (
            row.chars().nth(0).unwrap(),
            row.chars().nth(2).unwrap(),
        ))
        .collect()
}