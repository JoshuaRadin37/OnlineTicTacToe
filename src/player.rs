use crate::game::Move;

pub trait Player {

    fn my_move(&mut self) -> Move {
        unimplemented!()
    }
    fn send_move(&mut self, mov: &Move) -> std::io::Result<()>;
    fn enemy_move(&self) -> std::io::Result<Move>;


}