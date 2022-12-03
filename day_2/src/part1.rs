use std::fs;
use crate::input::get_raw_input;

#[derive(Eq, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    pub fn from(c: &char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("invalid input")
        }
    }

    pub fn shape_points(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    pub fn played_points(&self, other_shape: &Self) -> u8 {
        if self.eq(other_shape) {
            3
        } else {
            match (self, other_shape) {
                (Self::Rock, Self::Scissors) |
                (Self::Paper, Self::Rock) |
                (Self::Scissors, Self::Paper) => 6,
                _ => 0
            }
        }
    }
}

pub fn get_input() -> Vec<(Shape, Shape)> {
    get_raw_input().iter().map(|(a, b)| (Shape::from(a), Shape::from(b))).collect()
}

pub fn get_result() -> u32 {
    get_input().iter()
        .map(|(a, b)| {
            b.shape_points() + b.played_points(a)
        })
        .map(|a| a as u32)
        .sum()
}