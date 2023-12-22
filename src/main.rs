#![allow(non_snake_case)]
mod zixza;
mod collect_data;
mod load_agent;

use collect_data::{write_data, save_agent};
use load_agent::load_agent;
use zixza::{montecarlo::McAgent, board::DiceMove};
use zixza::randomagent::RandomAgent;

use crate::zixza::{Zixza, input_usize};

fn get_data_in_agent() {
    let loopnum = 2000000;
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
            let (next_state, reward, done, how_win) = game.step(action);
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
            game.reset();
            game.testset();
            agent.reset();
            let mut state = game.get_state();
            let mut data_buf: Vec<(u64, usize, usize)> = Vec::new();
            let mut count = 0;
            loop {
                count += 1;
                let actions = game.get_actions();
                let action = agent.get_action(&state, &actions);
                let (next_state, reward, done, how_win) = game.step(action);
                agent.add(state, action, reward, actions);
                data_buf.push((state, count, how_win));
                if done {
                    agent.update();
                    break;
                }
                state = next_state;
                
            }
            if i%10000==0{ println!("{}",i);}
            if i == (loopnum-1) {
                write_data(agent.get_memories());
                save_agent(agent.get_pi());
            }
        }
    }

}

fn mc_vs_player() {
    let mut game = Zixza::new();
    let pi = load_agent();
    let mut agent = McAgent::new();
    game.reset();
    game.testset();
    agent.reset();
    agent.load(pi);
    let mut player = true;
    let mut inmatch = false;
    let mut state = game.get_state();
    loop {
        let actions = game.get_actions();
        if player {
            let action = game.select_action();
            let (next_state, reward, done, how_to_win) = game.step(action);
            inmatch = done;
            agent.add(state, action, reward, actions);
            state = next_state;

        }else {
            let action = agent.get_action(&state, &actions);
            let (next_state, reward, done, how_to_win) = game.step(action);
            inmatch = done;
            agent.add(state, action, reward, actions);
            state = next_state;
        }
        if inmatch{
            break;
        }
        player = if player {false} else {true};
    }
}
fn first_code(){
    let mut game = Zixza::new();
    game.setup();
    game.start();
}
fn second_code(){
    let mut game = Zixza::new();
    game.reset();
    game.testset();
    let mut inmatch = false;
    let mut state = game.get_state();
    loop {
        let action = game.select_dice_and_action();
        let (next_state, reward, done, how_to_win) = game.step(action);
        inmatch = done;
        state = next_state;
        if inmatch{
            break;
        }
    }
}

fn main() {
    let a = input_usize();
    if a == 1{
        get_data_in_agent();
    }else if a == 2{
        mc_vs_player();
    }else if a == 3{
        first_code();
    }else {
        second_code();
    }
    //mc_vs_player();
    // get_data_in_agent();
}