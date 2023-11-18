#![allow(non_snake_case)]
mod zixza;
mod collect_data;

use collect_data::{write_data, save_agemt};
use zixza::montecarlo::McAgent;
use zixza::randomagent::RandomAgent;

use crate::zixza::{Zixza, input_usize};

fn main() {
    let loopnum = 100000;
    let mut game = Zixza::new();
    let flag = false;
    if flag{
        let mut agent = RandomAgent::new();
        for i in 0..loopnum {
        // game.setup();
        game.reset();
        game.testset();
        agent.reset();
        let mut state = game.get_state();
        loop {
            let actions = game.get_actions();
            let action = agent.get_action(actions);
            // println!("{}{}{}", action.0, action.1.to_string(), action.2);
            let (next_state, reward, done) = game.step(action);
            agent.add(state, action, reward);
            // println!("{}", done);
            // let i = input_usize();
            if done {
                agent.eval();
                break;
            }
            // println!("{:?}", state);
            state = next_state;
            
        }
        if i%10000==0{ println!("{}",i);}
        // game.show();
        }
        agent.q_show();
    }else{
        let mut agent = McAgent::new();
        for i in 0..loopnum {
            // game.setup();
            game.reset();
            game.testset();
            agent.reset();
            let mut state = game.get_state();
            loop {
                let actions = game.get_actions();
                let action = agent.get_action(&state, &actions);
                let (next_state, reward, done) = game.step(action);
                agent.add(state, action, reward, actions);
                // println!("{}{}{}", action.0, action.1.to_string(), action.2);
                // println!("{}", done);
                // let i = input_usize();
                if done {
                    agent.update();
                    
                    break;
                }
                // println!("{:?}", state);
                state = next_state;
                
            }
            if i%10000==0{ println!("{}",i);}
            // if i == 0{
            //     game.show();
            // }
            // 
            if i == (loopnum-1) {
                        write_data(agent.get_memories());
                        save_agemt(agent.get_pi());
                    }
        }
        // agent.q_show(10);
    }
    



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
