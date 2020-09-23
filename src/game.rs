use crate::game::GameResult::{Loss, Tie, Win};

pub enum Move {
    Rock,
    Paper,
    Scissors
}

pub enum GameResult {
    Win,
    Loss,
    Tie
}

impl Move {

    fn fight(&self, other: &Self) -> GameResult {
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

