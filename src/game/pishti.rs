
use std::collections::HashMap;

use crate::element::card::{Card, CardType, Color, DataContainer, Symbol, Tag};
use crate::element::player::Player;
use crate::game::Game;

enum GameCard{
    Card(Card),
    SecretCard(Card)
}

struct Pishti(GameCard, GameCard);

fn build_card() -> Vec<GameCard> {
    let mut cards: Vec<GameCard> = Vec::new();

    for i in [Color::Black, Color::Red] {
        for k in [Symbol::Cup, Symbol::Fly, Symbol::Spade, Symbol::Tile] {
            for j in 2..11{
                if k == Symbol::Tile && j == 10 {
                    cards.push(
                        GameCard::Card(
                            Card::new(
                                CardType::new(
                                    i.clone(),
                                    k.clone()),
                                DataContainer::Normal(j),
                                Tag::Point(3)
                            )
                        )
                    );
                    continue
                }

                else if k == Symbol::Fly && j == 2 {
                    cards.push(
                        GameCard::Card(
                            Card::new(
                                CardType::new(
                                    i.clone(),
                                    k.clone()),
                                DataContainer::Normal(j),
                                Tag::Point(2)
                            )
                        )
                    );
                    continue
                }
                cards.push(
                    GameCard::Card(
                        Card::new(
                            CardType::new(
                                i.clone(),
                                k.clone()),
                            DataContainer::Normal(j),
                            Tag::Nothingness
                        )
                    )
                );
            }

            for j in ['J', 'A'] {
                cards.push(
                    GameCard::Card(
                        Card::new(
                            CardType::new(
                                i.clone(),
                                k.clone()),
                            DataContainer::Special(j),
                            Tag::Point(1)
                        )
                    )
                );
            }

            for j in ['Q', 'K'] {
                cards.push(
                    GameCard::Card(
                        Card::new(
                            CardType::new(
                                i.clone(),
                                k.clone()),
                            DataContainer::Special(j),
                            Tag::Nothingness
                        )
                    )
                );
            }
        }
    }

    cards
}

struct GameSide {
    laid_cards: Vec<GameCard>, // ortaya koyulan
    dealt_cards: Vec<GameCard>, // dağıtılan
    players: Vec<Player>,
}

impl GameSide {
    fn new(players: Vec<Player>) -> Self {
        Self {
            laid_cards: Vec::new(),
            dealt_cards: build_card(),
            players
        }
    }
}

struct PlayerSide {
    order_of_play: HashMap<Player, usize>,
    cards_player_collected: HashMap<Player, Vec<GameCard>>,
    pishtis_collected_by_the_player: HashMap<Player, Vec<Pishti>>
}

impl PlayerSide {
    fn new() -> Self {
        Self {
            order_of_play: HashMap::new(),
            cards_player_collected: HashMap::new(),
            pishtis_collected_by_the_player: HashMap::new()
        }
    }
}

pub struct PishtiGame{
    game_side: GameSide,
    player_side: PlayerSide
}

impl PishtiGame {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            game_side: GameSide::new(players),
            player_side: PlayerSide::new()
        }
    }
}

impl Game for PishtiGame {
    fn handle(&self, player: Player) {
        println!("------------------------------------- {} it's your turn -------------------------------------", player.get_name());

        println!("cards in hand:");
        for i in 0..player.get_deck().len() {
            print!("Card{}:     ",i+1)
        }
        print!("\n");
        for i in 0..player.get_deck().len() {
            print!("{}", match player.get_deck()[i].get_card_type().get_color() {
                Color::Black => "Black",
                Color::Red => "Red  ",
            });
            print!("      ");
        }
        print!("\n");
        for i in 0..player.get_deck().len() {
            print!("{}          ", match player.get_deck()[i].get_card_type().get_symbol() {
                Symbol::Tile => "♢",
                Symbol::Fly => "♧",
                Symbol::Spade => "♤",
                Symbol::Cup => "♡"
            });
        }
        print!("\n");
        for i in 0..player.get_deck().len() {
            print!("{}          ", match player.get_deck()[i].get_identity() {
                DataContainer::Normal(a) => a.to_string(),
                DataContainer::Special(a) => a.to_string()
            });
        }
    }
    
    fn play(&self) {

    } 
}
