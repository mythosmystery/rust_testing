use rand::Rng;
use std::fmt::{Display, Debug, Formatter, Result};
use std::vec::Vec;

#[derive(Debug)]
pub enum Suit {
    SPADES,
    CLUBS,
    HEARTS,
    DIAMONDS,
}

impl Suit {
    pub fn rand() -> Suit {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Suit::CLUBS,
            1 => Suit::DIAMONDS,
            2 => Suit::HEARTS,
            _ => Suit::SPADES,
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub value: i8,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let card_value =
            match self.value {
                1 => String::from("Ace"),
                11 => String::from("Jack"),
                12 => String::from("Queen"),
                13 => String::from("King"),
                _ => self.value.to_string()
            };
        write!(f, "{} of {:?}", card_value, self.suit)
    }
}

impl Card {
    pub fn rand() -> Card {
        let mut rng = rand::thread_rng();
        Card {
            value: rng.gen_range(1..14),
            suit: Suit::rand(),
        }
    }
}

pub fn print_cards(cards: &Vec<Card>) {
    print!("|");
    for card in cards {
        print!(" {} |", card);
    }
    print!("\n");
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: Vec::new() };
        for _ in 0..56 {
            deck.cards.push(Card::rand());
        }
        deck
    }

    pub fn shuffle(self) -> Deck {
        Deck::new()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}
