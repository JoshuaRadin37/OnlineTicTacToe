use crate::game::Move;

pub trait Player {

    fn my_move(&mut self) -> Move;
    fn enemy_move(&self, enemy: &mut Self) -> Move;


}