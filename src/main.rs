mod zixza;
use zixza::board::DiceMove;

use crate::zixza::Zixza;

fn main() {
    let mut game = Zixza::new();
    game.setup();
    let state = game.get_state();
    let actionlist = game.get_actions();
    let mut done = false;
    while !done {
        let next_state;
        let reward;
        let action = (0,DiceMove::BackwardLeft,0);
        // let action = agent.select_action(game.get_actions());
        (next_state, reward, done) = game.step(action);
    }
    // game.setplayertype(zixza::PlayerType::MonteCarlo, zixza::PlayerType::MonteCarlo);
    
    // game.start();
}
