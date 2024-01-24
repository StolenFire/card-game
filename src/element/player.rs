use crate::element::card::Card;

#[derive(Debug)]
pub struct Player{
    name: String,
    deck: Vec<Card>,
}

impl Player {
    pub fn new(name: String, deck: Vec<Card>) -> Self {
        Self{ name, deck }
    }
    pub fn get_name(&self) -> String { self.name.clone() }
    pub fn get_deck(&self) -> Vec<Card> { self.deck.clone() }

}
