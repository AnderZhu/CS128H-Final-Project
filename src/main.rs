#[macro_use] extern crate rocket;

use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use black_jack::message::Message;
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

pub mod black_jack;
use black_jack::runner::BlackJackRunner;
use crate::black_jack::player::Player;
use crate::black_jack::deck::Deck;

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    unsafe { if STATE != 0 { state(STATE, form.message.clone()); }}
    if form.message.clone() == "Dealer" {
        unsafe { RESPOND = 15; }
    } else if form.message.clone() == "State" {
        unsafe { RESPOND = 404; }
    }
    let _res = queue.send(form.into_inner());
}

#[get("/game")]
fn game() {
    let mut blackjack_runner = BlackJackRunner::new();
    blackjack_runner.run();
}

static mut STATE: u32 = 0;
static mut RESPOND: u32 = 0;
static mut ITERATOR: usize = 0;
static mut N_PLAYER: usize = 0;
static mut PLAYERS: Vec<Player> = vec![];
static mut DECK: Vec<Deck> = vec![];

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);

            unsafe {
            if (RESPOND == 0 && msg.message == "blackjack") {
                STATE = 1;
                yield respond(1);
            }
            if (RESPOND != 0) {
                if (msg.message == "quit") {
                    STATE = 10;
                    yield respond(0);
                } else {
                    yield respond(RESPOND);
                }
            }}
        }
    }
}

fn state(state: u32, input: String) {
    unsafe {
    if state == 1 { // Start
        STATE = 2;
        RESPOND = 2;
    } else if state == 2 { // Ask for number of decks
        match input.parse::<usize>() {
            Ok(val) => {
                if val >= 6 && val <= 8 {
                    STATE = 3;
                    RESPOND = 3;
                    DECK.push(Deck::new(val));
                } else {
                    RESPOND = 10;
                }
            }
            Err(_) => {
                RESPOND = 11;
            }
        }
    } else if state == 3 { // Game start
        STATE = 4;
        RESPOND = 4
    } else if state == 4 { // Ask for number of players
        match input.parse::<usize>() {
            Ok(val) => {
                if val >= 1 && val <= 7 {
                    STATE = 5;
                    RESPOND = 5;
                    N_PLAYER = val;
                } else {
                    RESPOND = 12;
                }
            }
            Err(_) => {
                RESPOND = 11;
            }
        }
    } else if state == 5 { // Establish new players
        if ITERATOR < N_PLAYER {
            PLAYERS.push(Player::new(String::from(input.trim())));
            PLAYERS[ITERATOR + 1].initial_r(&mut DECK[0]);
            ITERATOR += 1;
        }
        if ITERATOR >= N_PLAYER { // Initialize dealer
            PLAYERS[0].initial_r(&mut DECK[0]);
            STATE = 6;
            RESPOND = 6;
            ITERATOR = 0;
        }
    } else if state == 6 { //
        STATE = 7;
        RESPOND = 7;
    } else if state == 10 { // Abort
        STATE = 0;
        RESPOND = 0;
    } else { // Unexpected error
        RESPOND = 404;
    }}
}

fn respond(respond: u32) -> Event {
    unsafe {
    if respond == 0 {
        bot("Game aborted.")
    } else if respond == 1 {
        bot("Welcome to BlackJack! Enter \"quit\" to leave the game\t(Type anything to continue)")
    } else if respond == 2 {
        bot("How many decks do you wanna use? (6-8)")
    } else if respond == 3 {
        bot("####### Game Started! #######\t(Type anything to continue)")
    } else if respond == 4 {
        bot("How many players are playing? (1-7)")
    } else if respond == 5 {
        bot(format!("Player {}, please, enter your name.", ITERATOR + 1).as_str())
    } else if respond == 6 {
        bot(format!("The first card of the dealer is {}\t(Type anything to continue)", PLAYERS[0].get_hand()[0]).as_str())
    } else if respond == 7 {
        bot("")
    } else if respond == 10 {
        bot("The number of decks must be between 6 and 8")
    } else if respond == 11 {
        bot("Expect integer input")
    } else if respond == 12 {
        bot("The number of players must be between 1 and 7")
    } else {
        bot(format!("Error! STATE: {}\tRESPOND: {}\t Please restart program.", STATE, RESPOND).as_str())
    }}
}

fn bot(str: &str) -> Event {
    let send = Message{room: String::from("lobby"),
                               username: String::from("bot"),
                               message: String::from(str)};
    return Event::json(&send);
}

#[launch]
fn rocket() -> _ {
    unsafe { PLAYERS.push(Player::new(String::from("Dealer"))); }
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events, game])
        .mount("/", FileServer::from(relative!("static")))
}
