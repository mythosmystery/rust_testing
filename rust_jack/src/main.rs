use colored::*;
use figlet_rs::FIGfont;

mod deck;
use deck::Deck;

use std::io::stdin;

mod player;
use player::Player;

mod game;

fn main() {
    welcome();

    let mut player = Player::new();
    let mut dealer = Player::new();

    println!("Enter your name: ");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    player.name = name;
    println!("Welcome {}", player.name);

    game::game_loop(&mut player, &mut dealer);
}

fn welcome() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("Welcome to Rust Jack!");
    assert!(figure.is_some());
    println!("---------------------------------------------------------------------------------------------------------------------------------------------");
    println!("{}", figure.unwrap());
    println!("---------------------------------------------------------------------------------------------------------------------------------------------");
}
