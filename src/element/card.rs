use std::fmt::Display;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Color {
    Red,
    Black
}
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Symbol {
    Cup,
    Tile,
    Spade,
    Fly
}
#[derive(Debug)]
#[derive(Clone)]
pub enum DataContainer {
    Normal(u8),
    Special(char)
}
#[derive(Clone)]
#[derive(Debug)]
pub enum Tag{
    Nothingness,
    Point(usize)
}
#[derive(Debug)]
#[derive(Clone)]
pub struct CardType {
    color: Color,
    symbol: Symbol,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Card {
    card_type: CardType,
    identity: DataContainer,
    tag: Tag
}

impl CardType {
    pub fn new(color: Color, symbol: Symbol) -> Self {
        Self{ color, symbol }
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }

}

impl Card {
    pub fn new(card_type: CardType, identity: DataContainer, tag: Tag) -> Self {
        Self{ card_type, identity, tag }
    }

    pub fn get_card_type(&self) -> CardType {
        self.card_type.clone()
    }

    pub fn get_identity(&self) -> DataContainer {
        self.identity.clone()
    }

    pub fn get_tag(&self) -> Tag {
        self.tag.clone()
    }

}

