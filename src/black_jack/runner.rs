use std::io;
use std::io::Write;

use crate::black_jack::card::Card;
use crate::black_jack::deck::Deck;
use crate::black_jack::player::Player;

use super::deck;

/// Runs a BlackJack game on the command line.
#[derive(Clone, Default)]
pub struct BlackJackRunner {
    players: Vec<Player>,
    deck: Deck,
    dealer: Player,
}

impl BlackJackRunner {
    pub fn new() -> BlackJackRunner {
        BlackJackRunner {
            players: vec![],
            deck: Deck::default(),
            dealer: Player::new(String::from("Dealer")),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to BlackJack!\n");

        let deck_n: usize = loop {
            match ask_input("How many decks do you wanna use? (6-8)")
                .trim()
                .parse()
            {
                Ok(val) => {
                    if val >= 6 && val <= 8 {
                        break val;
                    } else {
                        println!("The number of decks must be between 6 and 8");
                    }
                }
                Err(_) => {
                    println!("Expect integer input");
                }
            }
        };
        self.deck = Deck::new(deck_n);

        println!("\n####### Game Started! #######\n");

        let player_n: usize = loop {
            match ask_input("How many players are playing? (1-7)")
                .trim()
                .parse()
            {
                Ok(val) => {
                    if val >= 1 && val <= 7 {
                        break val;
                    } else {
                        println!("The number of decks must be between 6 and 8");
                    }
                }
                Err(_) => {
                    println!("Expect integer input");
                }
            }
        };
        ask_set_player_attributes(player_n, &mut self.players, &mut self.deck);
        set_dealer(&mut self.dealer, &mut self.deck);
        println!(
            "\nThe first card of the dealer is {}\n",
            &self.dealer.get_hand()[0]
        );

        loop {
            for player in self.players.iter_mut() {
                player_turn(player, &mut self.deck, false);
            }

            // end_game(&mut players, &dealer_hand);
            // if !next_game(&mut players, &mut dealer_hand, &mut deck) {
            //     break;
            // }
        }
    }
}
fn set_dealer(dealer: &mut Player, deck: &mut Deck) {
    dealer.initial_r(deck);
}

fn ask_set_player_attributes(player_n: usize, players: &mut Vec<Player>, deck: &mut Deck) {
    for i in 0..player_n {
        let name = ask_input(format!("\nPlease, enter your name player #{}", i + 1).as_str());

        players.push(Player::new(String::from(name.trim())));
        players[i].initial_r(deck);
    }
}

fn player_turn(player: &mut Player, deck: &mut Deck, dealer: bool) {
    let initial_cards = player.get_hand();
    println!(
        "\nYour cards are:\n{} and {} ({} points)\n",
        initial_cards[0],
        initial_cards[1],
        player.get_score()
    );
    while !win_or_lose(player) {
        match ask_input("What do you want to do?\nAvailable Commands: (h)it, (s)tand")
            .to_lowercase()
            .trim()
        {
            "h" | "hit" => {
                player.hit(deck);
                println!("Now, the cards are: ");
                for card in player.get_hand() {
                    println!("{}", card);
                }
                println!("{}", player.get_score());
            }
            "s" | "stand" => {
                println!("{} stood", player);
                break;
            }

            _ => println!("Invalid command!\nAvailable Commands: (h)it, (s)tand"),
        }
    }
}

//if player has won or lost
fn win_or_lose(player: &mut Player) -> bool {
    false
}

fn ask_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}\n>", prompt);
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut input).expect("Failed to read");
    input
}
