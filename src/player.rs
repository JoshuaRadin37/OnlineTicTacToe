use crate::game::Move;
use std::io::{BufReader, stdin, BufRead, stdout, Write};
use std::convert::TryInto;

pub trait Player {

    fn my_move(&mut self) -> Move {
        let mut reader = BufReader::new(stdin());
        loop {
            let mut buffer = String::new();
            print!("Your move (rock, paper, scissors): ");
            stdout().flush().expect("Failed to flush stdout");
            reader.read_line(&mut buffer).expect("Failed to get player's move");
            let mov = buffer.trim_end();

            if let Ok(ret) = mov.to_string().try_into() {
                return ret;
            }
        }
    }
    fn send_move(&mut self, mov: &Move) -> std::io::Result<()>;
    fn enemy_move(&self) -> std::io::Result<Move>;

    fn my_name(&self) -> &str;
    fn enemy_name(&self) -> Option<&str>;

}