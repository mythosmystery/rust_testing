use crate::deck::Card;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Player {
  pub name: String,
  pub cards: Vec<Card>,
  pub wins: u32,
  pub loses: u32,
}

impl Player {
  pub fn new() -> Player {
    Player {
      name: String::new(),
      cards: vec![Card::rand(), Card::rand()],
      wins: 0,
      loses: 0
    }
  }

  pub fn score_hand(&self) -> i8 {
    let mut total = 0;
    for card in &self.cards {
      total += match card.value {
        11 => 10,
        12 => 10,
        13 => 10,
        1 => if total + 11 > 21 { 1 } else { 11 },
        _ => card.value
      };
    };
    total
  }
}