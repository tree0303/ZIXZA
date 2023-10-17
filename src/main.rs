mod zixza;
use zixza::montecarlo::McAgent;
use zixza::randomagent::RandomAgent;

use crate::zixza::{Zixza, input_usize};

fn main() {
    let loopnum = 100000;
    let mut game = Zixza::new();
    // let mut agent = McAgent::new();
    let mut agent = RandomAgent::new();
    for i in 0..loopnum {
        // game.setup();
        game.reset();
        game.testset();
        agent.reset();
        let mut state = game.get_state();
        let mut count = 0;
        loop {
            count += 1;
            let actions = game.get_actions();
            let action = agent.get_action(actions);
            // println!("{}{}{}", action.0, action.1.to_string(), action.2);
            let (next_state, reward, done) = game.step(action);
            agent.add(state.iter().map(|v| *v as u64).collect(), action, reward);
            // println!("{}", done);
            // let i = input_usize();
            if done {
                agent.eval();
                break;
            }
            // println!("{:?}", state);
            state = next_state;
            
        }
        // println!("count{}",count);
        if i%10000==0{ println!("{}",i);}
        // game.show();
    }
    agent.q_show();



    // let mut game = Zixza::new();
    // let mut agent = McAgent::new();
    // game.setup();
    // let state = game.get_state();
    // let mut done = false;
    // while !done {
    //     let next_state;
    //     let reward;
    //     // let action = (0,DiceMove::BackwardLeft,0);
    //     // let action = agent.get_actions(game.get_actions());
    //     (next_state, reward, done) = game.step(action);
    // }
    // // game.setplayertype(zixza::PlayerType::MonteCarlo, zixza::PlayerType::MonteCarlo);
    
    // // game.start();
}
