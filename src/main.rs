use black_jack::runner::BlackJackRunner;

pub mod black_jack;
pub mod chat_server;
fn main() {
    let mut blackjack_runner = BlackJackRunner::new();
    blackjack_runner.run();
}
