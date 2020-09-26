use crate::game::GameResult::{Loss, Tie, Win};
use std::fmt::{Display, Formatter};
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}



impl TryFrom<String> for Move {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match & *s {
            "rock" => Ok(Move::Rock),
            "paper" => Ok(Move::Paper),
            "scissors" => Ok(Move::Scissors),
            _ => Err(())
        }
    }
}


impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Move::Rock => "rock",
            Move::Paper => "paper",
            Move::Scissors => "scissors"
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

