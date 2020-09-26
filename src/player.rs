use crate::game::Move;
use std::convert::TryInto;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

/// The functions needed for a player to be able to interact in the game with another player
pub trait Player {
    /// Gets the move made by this player. The default implementation that is used by both
    /// the client and the server is to get the move through the standard input
    fn my_move(&mut self) -> Move {
        let mut reader = BufReader::new(stdin());
        loop {
            let mut buffer = String::new();
            print!("Your move (rock, paper, scissors): ");
            stdout().flush().expect("Failed to flush stdout");
            reader
                .read_line(&mut buffer)
                .expect("Failed to get player's move");
            let mov = buffer.trim_end();

            if let Ok(ret) = mov.to_string().try_into() {
                return ret;
            }
        }
    }
    /// Sends the move that this player made to the opposing player
    ///
    /// # Error
    /// This function will return an error if the enemy player has disconnected from this player
    fn send_move(&mut self, mov: &Move) -> std::io::Result<()>;

    /// Waits for the enemy player to send a move to be received. This function is blocking.
    ///
    /// # Error
    /// This function will return an error if the enemy player has disconnected
    fn enemy_move(&self) -> std::io::Result<Move>;

    /// Gets the name of this player
    fn my_name(&self) -> &str;

    /// Gets the name of the enemy, if the enemy exists
    fn enemy_name(&self) -> Option<&str>;
}
