
/*
Oyun:
    kural1: kart sayısında değişiklik olmaz.(52)
    kural2: oyuncu sayısı, 12'yi kalansız bölebilmeli.
    kural3: saat yönünün tersine göre ilk başlayan kişi sırası her oyun yeniden başladığında değişir.
    kural4: oyuna ilk başlanıldığında ilk 3 kart kapalı şekilde ortaya konulmalı ve bir tane de açık konulmalı.
    kural5: her oyuncuya sadece 4 kart verilir
    kural6: sırayla oyuna başlanır ve en son koyulan karttaki sembol ile uyuşan bir kart atılırsa
        eğer altında uyuşan kart dışında başka kart yok ise pişti olur ve karta göre puan alır
        eğer altında uyuşan kart dışında kart var ise oradaki bütün kartları alır
    kural7: joker, as, sinek 2 ve karo 10 puanlı kartlardır.
    kural8: kart çokluğu 3 puandır
    kural9: joker ve as 1 puandır, sinek 2, 2 puan ve karo 10, 3 puandır.
    kural10: 48 kartın hepsi bitene kadar oyuna devam edilir.
    kural11: oyun sonunda toplanan kart ve piştilerle puan hesaplanır ve kazanan belirlenir
 */

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

/* Handler şöyle olmalı

Sırayla atılan kartları ve olayları görmeli
Sonra sıra ona geldiğinde Diğerlerin topladığı kartlara bakarak ve neler atıldığına bakmalı
Sonra kendi topladığı kartlara bakabilmeli
Ondan Sonra da kartını seçip atabilmeli

*/

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
