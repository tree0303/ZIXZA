use zixza::{Zixza, montecarlo::McAgent, board::DiceMove};
use wasm_bindgen::prelude::*;
mod load_agent;
use load_agent::load_agent;

mod zixza;
mod collect_data;

pub fn game_start() {
    let pi = load_agent().expect("could not load agent");
    let mut game = Zixza::new();
    let mut agent = McAgent::new();
    game.reset();
    game.testset();
    agent.reset();
    agent.load(pi);
    let mut state = game.get_state();
    let mut is_first_player = true;// jsから先手後手を受信
    let mut in_match = false;
    loop {
        // jsにstateを送信
        if is_first_player {
            let actions = game.get_actions();
            // actionsをjsに送信
            // actionをjsから受信
            let action = (1, DiceMove::TurnLeft, 1);
            let (next_state, reward, done, how_win) = game.step(action);
            in_match = done;
            agent.add(state, action, reward, actions);
            state = next_state;
        } else {
            let actions = game.get_actions();
            let action = agent.get_action(&state, &actions);
            let (next_state, reward, done, how_win) = game.step(action);
            in_match = done;
            agent.add(state, action, reward, actions);
            state = next_state;
        }
        if in_match {
            break;
        }
        is_first_player = if is_first_player {false} else {true};
    }
}


#[wasm_bindgen]
pub fn start() -> u64{
    let pi = load_agent().expect("could not load agent");
    let mut game = Zixza::new();
    let mut agent = McAgent::new();
    game.reset();
    game.testset();
    agent.reset();
    agent.load(pi);
    let mut state = game.get_state();
    let mut is_first_player = false;// jsから先手後手を受信
    let mut in_match = false;
    loop {
        // jsにstateを送信
        if is_first_player {
            let actions = game.get_actions();
            // actionsをjsに送信
            // actionをjsから受信
            let action = (1, DiceMove::TurnLeft, 1);
            let (next_state, reward, done, how_win) = game.step(action);
            in_match = done;
            agent.add(state, action, reward, actions);
            state = next_state;
        } else {
            let actions = game.get_actions();
            let action = agent.get_action(&state, &actions);
            let (next_state, reward, done, how_win) = game.step(action);
            in_match = done;
            agent.add(state, action, reward, actions);
            state = next_state;
        }
        if in_match {
            break;
        }
        // is_first_player = if is_first_player {false} else {true};
    }
    return state;
}





#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn say(name: &str) {
    alert(&format!("Hello, {}!", name));
}