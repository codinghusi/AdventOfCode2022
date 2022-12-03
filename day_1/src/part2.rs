use std::fs;

pub fn get_result() -> u32 {
    let mut vec = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|elf|
            elf.split("\n")
                .map(|food| food.parse::<u32>().unwrap())
                .sum()
        )
        .collect::<Vec<u32>>();
    vec.sort_by(|a, b| b.cmp(a));
    vec.iter()
        .take(3)
        .sum()
}