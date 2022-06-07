use crate::deck;
use crate::player::Player;
use std::io::stdin;

pub fn game_loop(player: &mut Player, dealer: &mut Player) {
  loop {
    println!("Your cards are: ");
    deck::print_cards(&player.cards);
    println!("Score of: {}", player.score_hand());
    println!("Dealer's card is: {}", &dealer.cards[0]);

    println!("Would you like to play again? (y/n)");
    let mut play_again = String::new();
    stdin().read_line(&mut play_again).unwrap();
    if play_again.trim() == "n" {
      break;
    }
    player.new_hand();
    dealer.new_hand();
  }
}
