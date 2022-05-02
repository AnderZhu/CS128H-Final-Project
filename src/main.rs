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

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    unsafe { if STATE != 0 { state(STATE, form.message.clone()); }}
    let _res = queue.send(form.into_inner());
}

#[get("/game")]
fn game() {
    let mut blackjack_runner = BlackJackRunner::new();
    blackjack_runner.run();
}

static mut STATE: u32 = 0;
static mut RESPOND: u32 = 0;
static mut DECK: u32 = 0;
static mut PLAYER: u32 = 0;

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
        match input.parse::<u32>() {
            Ok(val) => {
                if val >= 6 && val <= 8 {
                    STATE = 3;
                    RESPOND = 3;
                    DECK = val;
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
        match input.parse::<u32>() {
            Ok(val) => {
                if val >= 1 && val <= 7 {
                    STATE = 5;
                    RESPOND = 5;
                    PLAYER = val;
                } else {
                    RESPOND = 12;
                }
            }
            Err(_) => {
                RESPOND = 11;
            }
        }
    } else if state == 5 { //

    } else if state == 10 { // Abort
        STATE = 0;
        RESPOND = 0;
    } else { // Unexpected error
        RESPOND = 404;
    }}
}

fn respond(state: u32) -> Event {
    if state == 0 {
        bot("Game aborted.")
    } else if state == 1 {
        bot("Welcome to BlackJack! Enter \"quit\" to leave the game")
    } else if state == 2 {
        bot("How many decks do you wanna use? (6-8)")
    } else if state == 3 {
        bot("####### Game Started! #######")
    } else if state == 4 {
        bot("How many players are playing? (1-7)")
    } else if state == 5 {
        bot("")
    } else if state == 10 {
        bot("The number of decks must be between 6 and 8")
    } else if state == 11 {
        bot("Expect integer input")
    } else if state == 12 {
        bot("The number of players must be between 1 and 7")
    } else {
        unsafe { bot(format!("Error! STATE: {}\tRESPOND: {}", STATE, RESPOND).as_str()) }
    }
}

fn bot(str: &str) -> Event {
    let send = Message{room: String::from("lobby"),
                               username: String::from("bot"),
                               message: String::from(str)};
    return Event::json(&send);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events, game])
        .mount("/", FileServer::from(relative!("static")))
}
