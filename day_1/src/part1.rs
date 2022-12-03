use std::fs;

pub fn get_result() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|food| food.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}