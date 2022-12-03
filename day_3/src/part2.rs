use std::collections::HashSet;
use itertools::Itertools;
use std::fs;


struct Item(pub char);

impl Item {
    pub fn priority(&self) -> u16 {
        let c = self.0;
        let code = c as u16;
        match c {
            'a'..='z' => code - ('a' as u16) + 1,
            'A'..='Z' => code - ('A' as u16) + 27,
            _ => panic!("invalid input")
        }
    }
}

pub fn part2() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .tuples()
        .map(|(elf1, elf2, elf3)| {
            for c in elf1.chars() {
                if elf2.contains(c) && elf3.contains(c) {
                    return c;
                }
            }
            panic!("invalid input")
        })
        .map(|c| Item(c))
        .map(|item| item.priority() as u32)
        .sum()
}