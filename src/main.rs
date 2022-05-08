use sea_wars::board::BoardConfig;
use sea_wars::player::{HumanPlayer, Player};

fn main() {
    let config = BoardConfig::new(10, 10, 1, 2, 2);
    let human = HumanPlayer::new(String::from("Enn3"), &config);
    println!("{}", human.board().string_value(true, true));
}
