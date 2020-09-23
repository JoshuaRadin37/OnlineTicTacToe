use crate::game::GameResult::{Loss, Tie, Win};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl From<String> for Move {
    fn from(s: String) -> Self {
        match & *s {
            "Rock" => Move::Rock,
            "Paper" => Move::Paper,
            "Scissors" => Move::Scissors,
            _ => panic!("Not a valid move")
        }
    }
}


impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Move::Rock => "Rock",
            Move::Paper => "Paper",
            Move::Scissors => "Scissors"
        };
        write!(f, "{}", s)
    }
}

pub enum GameResult {
    Win,
    Loss,
    Tie
}

impl Move {

    pub fn fight(&self, other: &Self) -> GameResult {
        match (self, other) {
            (Move::Rock, Move::Paper) => Loss,
            (Move::Paper, Move::Paper) => Tie,
            (Move::Scissors, Move::Paper) => Win,
            (Move::Rock, Move::Rock) => Tie,
            (Move::Paper, Move::Rock) => Win,
            (Move::Scissors, Move::Rock) => Loss,
            (Move::Rock, Move::Scissors) => Win,
            (Move::Paper, Move::Scissors) => Loss,
            (Move::Scissors, Move::Scissors) => Tie
        }
    }

}

