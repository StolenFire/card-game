use crate::element::player::Player;
pub mod pishti;

pub trait Game {
    fn handle(&self, player: Player);
    fn play(&self);
}
