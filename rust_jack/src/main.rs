use colored::*;

mod deck;
use deck::Deck;

fn main() {
    println!(
        "{} {}{} {}",
        "Welcome to my",
        "Black".black().bold(),
        "jack".red().bold(),
        "game!"
    );
    let mut deck = Deck::new();
    println!("{:#?}", deck);
    println!("{:#?}", deck.draw());
}
