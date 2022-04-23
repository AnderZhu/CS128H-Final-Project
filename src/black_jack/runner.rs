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
        self.deck.shuffle();

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
                        println!("The number of decks must be between 1 and 7");
                    }
                }
                Err(_) => {
                    println!("Expect integer input");
                }
            }
        };
        ask_set_player_attributes(player_n, &mut self.players, &mut self.deck);
        set_dealer(&mut self.dealer, &mut self.deck);
        let mut blackjack_on_game = false;
        //if someone blackjack
        // todo()

        println!(
            "\nThe first card of the dealer is {}\n",
            &self.dealer.get_hand()[0]
        );

        loop {
            for player in self.players.iter_mut() {
                player_turn(player, &mut self.deck, false);
                if player.has_black_jack() {
                    blackjack_on_game = true;
                    break;
                }
            }

            end_game(&mut self.players, &self.dealer, blackjack_on_game);
            // if !next_game(&mut players, &mut dealer_hand, &mut deck) {
            //     break;
            // }
            break;
        }
    }
}
fn end_game(players: &mut Vec<Player>, dealer: &Player, blackjack_on_game: bool) {
    println!("####### Game Finished #######\n");
    if dealer.has_black_jack() {
        for player in players {
            if player.has_black_jack() {
                println!("{player} tied! :|\n");
            }
            println!("{player} lost! :(\n",);
        }
        return;
    } else {
        //dealer dont have blackjack
        //if player have blackjack
        if (blackjack_on_game) {
            for player in players {
                if player.has_black_jack() {
                    println!("{player} won! :)\n",);
                } else {
                    println!("{player} lost! :(\n",);
                }
            }
        } else {
            // player dont have blackjack
            let dealer_points = dealer.get_score();
            for player in players.iter_mut() {
                let player_points = player.get_score();
                if player_points > dealer_points {
                    println!("{player} won! :)\n",);
                } else if player_points < dealer_points {
                    println!("{player} lost! :(\n",);
                } else {
                    println!("{player} tied! :|\n");
                }
            }
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
        match ask_input(
            format!(
                "{} What do you want to do?\nAvailable Commands: (h)it, (s)tand",
                player
            )
            .as_str(),
        )
        .to_lowercase()
        .trim()
        {
            "h" | "hit" => {
                player.hit(deck);
                println!("Now, the cards are: ");
                for card in player.get_hand() {
                    println!("{}", card);
                }
                println!("Now you got {} points", player.get_score());
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
    if player.has_black_jack() {
        println!("BLACKJACK!\n");
        return true;
    } else {
        if player.get_score() == 21 {
            println!("YOU GOT 21 POINTS!\n");
            return true;
        }
        if player.get_score() > 21 {
            println!("BUST.\nI'm afraid you lose this game :(\n");
            return true;
        }
    }
    false
}

fn ask_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}\n>", prompt);
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut input).expect("Failed to read");
    input
}
