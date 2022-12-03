use std::fs;
use crate::input::get_raw_input;

#[derive(Eq, PartialEq, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    pub fn from(c: &char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("invalid input")
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    pub fn result(&self, other_shape: &Self) -> RoundResult {
        if self.eq(other_shape) {
            RoundResult::Draw
        } else {
            match (self, other_shape) {
                (Self::Rock, Self::Scissors) |
                (Self::Paper, Self::Rock) |
                (Self::Scissors, Self::Paper) => RoundResult::Win,
                _ => RoundResult::Loss
            }
        }
    }
}

#[derive(Eq, PartialEq)]
enum RoundResult {
    Loss,
    Draw,
    Win
}

impl RoundResult {
    pub fn from(c: &char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("invalid input")
        }
    }

    pub fn my_move(&self, opponent: &Shape) -> Shape {
        match self {
            Self::Draw => opponent.clone(),
            Self::Win => match opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock
            },
            Self::Loss => match opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper
            }
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6
        }
    }
}

fn get_input() -> Vec<(Shape, RoundResult)> {
    get_raw_input().iter().map(|(a, b)| (Shape::from(a), RoundResult::from(b))).collect()
}

pub fn get_result() -> u32 {
    get_input().iter()
        .map(|(a, b)| {
            b.score() + b.my_move(a).score()
        })
        .map(|a| a as u32)
        .sum()
}