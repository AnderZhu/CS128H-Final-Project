#[macro_use] extern crate rocket;

use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use black_jack::message::Message;
// use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

pub mod black_jack;
use black_jack::runner::BlackJackRunner;

// #[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct Message {
//     #[field(validate = len(..30))]
//     pub room: String,
//     #[field(validate = len(..20))]
//     pub username: String,
//     pub message: String,
// }

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}

#[get("/game")]
fn game() {
    let mut blackjack_runner = BlackJackRunner::new();
    blackjack_runner.run();
}

static mut NUM: u32 = 0;
static mut DECK: u32 = 0;
static mut PLAYER: u32 = 0;
// static mut dealer: Player = Player::new(String::from("Dealer"));

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
                if (NUM == 0 && msg.message == "blackjack") {
                    NUM += 1;
                    yield bot("Welcome to BlackJack! Enter \"quit\" to leave the game");
                }
                if (NUM != 0) {
                    if (msg.message == "quit") {
                        NUM = 0;
                    } else {
                        yield blackjack(NUM, msg.message);
                        unsafe { if (NUM == 3) { 
                            yield bot("How many players are playing? (1-7)");
                            NUM += 1; }}
                    }
                }
            }
        }
    }
}

fn blackjack(state: u32, input: String) -> Event {
    if state == 1 {
        unsafe {
            NUM += 1;
        }
        bot("How many decks do you wanna use? (6-8)")
    } else if state == 2 {
        match input.parse::<u32>() {
            Ok(val) => {
                if val >= 6 && val <= 8 {
                    unsafe {
                        NUM += 1;
                        DECK = val;
                    }
                    bot("####### Game Started! #######")
                } else {
                    bot("The number of decks must be between 6 and 8")
                }
            }
            Err(_) => {
                bot("Expect integer input")
            }
        }
    } else if state == 4 {
        match input.parse::<u32>() {
            Ok(val) => {
                if val >= 1 && val <= 7 {
                    unsafe {
                        NUM += 1;
                        PLAYER = val;
                        bot("")
                    }
                } else {
                    bot("The number of players must be between 1 and 7")
                }
            }
            Err(_) => {
                bot("Expect integer input")
            }
        }
    } else {
        bot("error!")
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
