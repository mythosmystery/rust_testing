use rand::Rng;
use std::fmt::Debug;

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

impl Card {
    pub fn rand() -> Card {
        let mut rng = rand::thread_rng();
        Card {
            value: rng.gen_range(1..15),
            suit: Suit::rand(),
        }
    }
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

    pub fn shuffle(&mut self) {
        let shuffledDeck = Deck::new();
        *self = shuffledDeck;
        
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}
