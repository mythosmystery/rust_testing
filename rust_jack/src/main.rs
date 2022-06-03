use colored::*;

mod deck;
use deck::Deck;

use std::io::stdin;

mod player;
use player::Player;

fn main() {
    welcome();

    let mut player = Player::new();
    let dealer = Player::new();

    println!("Enter your name: ");
    let name = &mut String::new();
    stdin().read_line(name).unwrap();
    player.name = name.to_owned();
    println!("Welcome {}", player.name);

    println!("Your cards are: ");
    deck::print_cards(&player.cards);
    println!("Score of: {}", player.score_hand());
    println!("Dealer's cards are: ");
    deck::print_cards(&dealer.cards);
}

fn welcome() {
    println!(
        "{} {}{} {}",
        "Welcome to my",
        "Black".black().bold(),
        "jack".red().bold(),
        "game!"
    );
}