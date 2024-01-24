mod element;
mod entry_point;
mod game;

use crate::element::card::{Card, CardType, Color, DataContainer, Symbol, Tag};
use crate::element::player::Player;
use crate::game::pishti::{PishtiGame};
use crate::game::Game;

fn main() {
    let mut a = PishtiGame::new(Vec::new());
    let mut deck = Vec::new();

    deck.push(
        Card::new(
            CardType::new(
                Color::Black,
                Symbol::Cup
            ),
            DataContainer::Normal(2),
            Tag::Nothingness
        )
    );

    deck.push(
        Card::new(
            CardType::new(
                Color::Red,
                Symbol::Spade
            ),
            DataContainer::Special('J'),
            Tag::Nothingness
        )
    );

    deck.push(
        Card::new(
            CardType::new(
                Color::Black,
                Symbol::Spade
            ),
            DataContainer::Special('J'),
            Tag::Nothingness
        )
    );

    deck.push(
        Card::new(
            CardType::new(
                Color::Red,
                Symbol::Spade
            ),
            DataContainer::Special('J'),
            Tag::Nothingness
        )
    );

    let player = Player::new("ahmad".to_string(), deck);
    a.handle(player);

}