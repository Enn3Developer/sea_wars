use sea_wars::board::BoardConfig;
use sea_wars::player::{HumanPlayer, Player};
use sea_wars::Message;

fn run(mut players: Vec<Box<dyn Player>>) {
    loop {
        let len = players.len();
        // players[i] is the player currently making the move
        // players[j] is the enemy on which the player is currently making the move against
        for i in 0..len {
            for j in 0..len {
                if j == i {
                    continue;
                }

                let input_point = players[i].input(players[j].board());
                match players[j].hit(input_point) {
                    Message::NoHit => {}
                    Message::HitOne => {}
                    Message::HitTwo => {}
                    Message::Sunk(points) => players[i].add_points(points),
                }
            }
        }
    }
}

fn main() {
    let mut players: Vec<Box<dyn Player>> = vec![];

    let config = BoardConfig::new(10, 10, 1, 2, 2);
    let human = HumanPlayer::new(String::from("Enn3"), &config);

    players.push(Box::new(human));

    run(players);
}
