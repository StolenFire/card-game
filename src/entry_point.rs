use crate::element::player::Player;
use crate::game::Game;

pub struct EntryPoint<T: Game> {
    players: Vec<Player>,
    game: T
}

impl<T: Game> EntryPoint<T> {
    pub fn new(players: Vec<Player>, game: T) -> Self{ Self { players, game } }
    pub fn start(&self) {

    }
}
