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

pub fn part1() -> u32 {
    fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|row| {
            let left = &row[..row.len()/2];
            let right = &row[row.len()/2..];
            let index = left
                .find(|x| right.contains(x))
                .expect("couldn't find it");
            left.chars().nth(index).unwrap()
        })
        .map(|c| Item(c))
        .map(|item| item.priority() as u32)
        .sum()
}