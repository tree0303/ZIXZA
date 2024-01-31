#![allow(non_snake_case)]
mod zixza;
mod collect_data;
mod load_agent;

use std::vec;

use collect_data::{write_data, save_agent, mc_vs_random_data};
use load_agent::load_agent;
use zixza::{montecarlo::McAgent, board::DiceMove};
use zixza::randomagent::RandomAgent;

use crate::collect_data::{mc_vs_random_data_2, mc_vs_mc_data, mc_vs_random_data_ex};
use crate::load_agent::load_agent_into_filename;
use crate::zixza::{Zixza, input_usize};

const LOOPNUM: usize = 4000000;
fn get_data_in_agent_by_new_state() {
    let loopnum = LOOPNUM;
    let mut game = Zixza::new();
    let mut agent = McAgent::new();
    agent.param_set(loopnum);
    for i in 0..loopnum {
        'over: loop {
            game.reset();
            game.testset();
            agent.reset();
            let mut count = 0;
            'game: loop {
                count += 1;
                let actions = game.get_actions();
                let new_state = game.learn_get_state();
                let action = agent.get_action(&new_state, &actions);
                let (_, reward, mut done, how_win) = game.step(action);
                if action.1 != DiceMove::Path {
                    agent.add(new_state, action, reward, actions);
                } else if action.1 == DiceMove::Path {
                    done = true;
                }
                if done {
                    agent.update();
                    break 'game;
                }
                if count > 140 {
                    break 'game;
                }
            }
            if count > 140 {
                break 'over;
            } else {
                continue 'over;
            }
        }
        
        if i%10000==0{ println!("{}",i);}
        if i == (loopnum-1) {
            // write_data(agent.get_memories());
            save_agent(agent.get_pi());
        }
    }
}

fn get_data_in_agent() {
    let loopnum = LOOPNUM;
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
        agent.param_set(loopnum);
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
                // let new_state = game.learn_get_state();
                // let action = agent.get_action(&new_state, &actions);
                let action = agent.get_action(&state, &actions);
                let (next_state, reward, mut done, how_win) = game.step(action);
                if action.1 != DiceMove::Path {
                    // agent.add(new_state, action, reward, actions);
                    // data_buf.push((new_state, count, how_win));
                    agent.add(state, action, reward, actions);
                    data_buf.push((state, count, how_win));
                } else if action.1 == DiceMove::Path {
                    done = true;
                }
                if done {
                    agent.update();
                    break;
                }
                state = next_state;
                
            }
            if i%10000==0{ println!("{}",i);}
            if i == (loopnum-1) {
                // write_data(agent.get_memories());
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
    println!("start");
    loop {
        let actions = game.get_actions();
        // let new_state = game.learn_get_state();
        if player {
            let action = game.select_dice_and_action();
            let (next_state, reward, done, how_to_win) = game.step(action);
            inmatch = done;
            // agent.add(new_state, action, reward, actions);
            agent.add(state, action, reward, actions);
            state = next_state;

        }else {
            // let action = agent.get_action(&new_state, &actions);
            let action = agent.get_action(&state, &actions);
            let (next_state, reward, done, how_to_win) = game.step(action);
            inmatch = done;
            // agent.add(new_state, action, reward, actions);
            agent.add(state, action, reward, actions);
            state = next_state;
        }
        if inmatch{
            write_data(agent.get_memories());
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

fn mc_vs_mc(){
    let mut game = Zixza::new();
    let mut agent_1 = McAgent::new();
    let mut agent_2 = McAgent::new();
    // let file_names = ["agent_set1.txt","agent_set2.txt","agent_set3.txt","agent_set4.txt",
    // "agent_set5.txt","agent_set6.txt","agent_set7.txt","agent_set8.txt","agent_set9.txt"];
    let file_names = ["agent_set3.txt","agent_set9.txt"];
    let mut buf= Vec::new();
    println!("start");
    let pi_1 = load_agent_into_filename(file_names[0]);
    let pi_2 = load_agent_into_filename(file_names[1]);
    agent_1.load(pi_1);
    agent_2.load(pi_2);
    for i in 0..100{
        println!("{}",i);
        game.reset();
        game.testset();
        agent_1.reset();
        agent_2.reset();
        let mut mc_1 = if i <500 {true}else{false};
        let mut inmatch = false;
        let mut mc_win = false;
        let mut how = "Exception";
        // println!("start");
        'main_game: loop {
            let actions = game.get_actions();
            let state = game.get_state();
            if mc_1 {
                let action = agent_1.get_action(&state, &actions);
                let (_, _, done, how_win) = game.step(action);
                inmatch = done;
                if inmatch {
                    mc_win = true;
                    how = match how_win {
                        1 => "Draw",
                        2 => "Occupation",
                        3 => "Reace",
                        4 => "Attack",
                        _ => how,
                    }
                }
                if action.1 == DiceMove::Path {
                    inmatch = true;
                }
            }else {
                let action = agent_2.get_action(&state, &actions);
                let (_, _, done, how_win) = game.step(action);
                inmatch = done;
                if inmatch {
                    how = match how_win {
                        1 => "Draw",
                        2 => "Occupation",
                        3 => "Reace",
                        4 => "Attack",
                        _ => how,
                    }
                }
                if action.1 == DiceMove::Path {
                    inmatch = true;
                    mc_win = true;
                    how = match how_win {
                        1 => "Draw",
                        2 => "Occupation",
                        3 => "Reace",
                        4 => "Attack",
                        _ => how,
                    }
                }
            }
            if inmatch{
                break 'main_game;
            }
            if game.get_steps() > 200 {
                how = "over";
                break 'main_game; 
            }
            mc_1 = if mc_1 {false} else {true};
            
        }
        if i%100==0{
            println!("{}",game.get_steps());
        }
        let first = if i <500 {true}else{false};
        buf.push((mc_win, how, game.get_steps()));
    }
    mc_vs_mc_data("mc3_vs_mc9", buf);
    println!("end");
}

fn mc_vs_random(){
    let mut game = Zixza::new();
    let mut agent = McAgent::new();
    let mut random = RandomAgent::new();
    // let file_names = ["agent_set1.txt","agent_set2.txt","agent_set3.txt","agent_set4.txt","agent_set5.txt","agent_set6.txt","agent_set7.txt"];
    let file_names = ["agent_set8.txt"];
    for (num, file_name) in file_names.into_iter().enumerate(){
        if num !=0 { continue;}
        let pi = load_agent_into_filename(file_name);
        let mut buf: Vec<(bool, bool, &str, usize)> = Vec::new();
        println!("start");
        for i in 0..100{
            println!("{}",i);
            game.reset();
            game.testset();
            agent.reset();
            agent.load(pi.clone());
            let mut mc = if i <500 {true}else{false};
            let state_flag = if num >= 5 {false} else {true};
            let mut inmatch = false;
            let mut mc_win = false;
            let mut how = "Exception";
            // println!("start");
            'main_game: loop {
                let actions = game.get_actions();
                let state = if state_flag {game.get_state()} else {game.learn_get_state()};
                if mc {
                    let action = agent.get_action(&state, &actions);
                    let (_, reward, done, how_win) = game.step(action);
                    inmatch = done;
                    if inmatch {
                        mc_win = true;
                        how = match how_win {
                            1 => "Draw",
                            2 => "Occupation",
                            3 => "Reace",
                            4 => "Attack",
                            _ => how,
                        }
                    }
                    if action.1 == DiceMove::Path {
                        inmatch = true;
                    }
                }else {
                    let action = random.get_action(actions);
                    let (_, reward, done, how_win) = game.step(action);
                    inmatch = done;
                    if inmatch {
                        how = match how_win {
                            1 => "Draw",
                            2 => "Occupation",
                            3 => "Reace",
                            4 => "Attack",
                            _ => how,
                        }
                    }
                    if action.1 == DiceMove::Path {
                        inmatch = true;
                        mc_win = true;
                        how = match how_win {
                            1 => "Draw",
                            2 => "Occupation",
                            3 => "Reace",
                            4 => "Attack",
                            _ => how,
                        }
                    }
                }
                if inmatch{
                    break 'main_game;
                }
                if game.get_steps() > 200 {
                    how = "over";
                    break 'main_game; 
                }
                mc = if mc {false} else {true};
                
            }
            if i%100==0{
                println!("{}",game.get_steps());
            }
            let first = if i <500 {true}else{false};
            buf.push((first, mc_win, how, game.get_steps()));
        }
        mc_vs_random_data(file_name, buf);
        println!("end");
    }
}

fn mc_vs_random_2(){
    let mut game = Zixza::new();
    let mut agent = McAgent::new();
    let mut random = RandomAgent::new();
    // let file_names = ["agent_set1.txt","agent_set2.txt","agent_set3.txt","agent_set4.txt","agent_set5.txt","agent_set6.txt","agent_set7.txt"];
    let file_names = ["agent_set2.txt","agent_set3.txt","agent_set4.txt",
    "agent_set5.txt","agent_set9.txt"];
    // let file_names = ["agent_set1.txt","agent_set10.txt"];
    for (index, file_name) in file_names.into_iter().enumerate(){
        // if num !=0 { continue;}
        let pi = load_agent_into_filename(file_name);
        agent.load(pi);
        for i in 0..20000{
            println!("start");
            println!("{}",i);
            game.reset();
            game.testset();
            agent.reset();
            
            let mut mc = if i <10000 {true}else{false};
            // let state_flag = if index >= 5 {false} else {true};
            let state_flag = true;
            let mut inmatch = false;
            let mut mc_win = false;
            let mut how = "Exception";
            // println!("start");
            'main_game: loop {
                println!("{}",game.get_steps());
                let actions = game.get_actions();
                let state = if state_flag {game.get_state()} else {game.learn_get_state()};
                if mc {
                    let action = agent.get_action(&state, &actions);
                    let (_, _, done, how_win) = game.step(action);
                    inmatch = done;
                    if inmatch {
                        mc_win = true;
                        how = match how_win {
                            1 => "Draw",
                            2 => "Occupation",
                            3 => "Reace",
                            4 => "Attack",
                            _ => how,
                        }
                    }
                    if action.1 == DiceMove::Path {
                        inmatch = true;
                    }
                }else {
                    let action = random.get_action(actions);
                    let (_, _, done, how_win) = game.step(action);
                    inmatch = done;
                    if inmatch {
                        how = match how_win {
                            1 => "Draw",
                            2 => "Occupation",
                            3 => "Reace",
                            4 => "Attack",
                            _ => how,
                        }
                    }
                    if action.1 == DiceMove::Path {
                        inmatch = true;
                        mc_win = true;
                        how = match how_win {
                            1 => "Draw",
                            2 => "Occupation",
                            3 => "Reace",
                            4 => "Attack",
                            _ => how,
                        }
                    }
                }
                if inmatch{
                    break 'main_game;
                }
                // if game.get_steps() > 200 {
                //     how = "over";
                //     break 'main_game; 
                // }
                mc = if mc {false} else {true};
            }
            let first = if i <10000 {true}else{false};
            mc_vs_random_data_ex(file_name,index+2,first, mc_win, how, game.get_steps());
        }
        
        println!("end");
    }
}


fn main() {
    println!("input");
    let a = input_usize();
    if a == 1{
        get_data_in_agent();

    }else if a == 2 {
        get_data_in_agent_by_new_state();

    }else if a == 3{
        mc_vs_player();

    }else if a == 4{
        first_code();

    }else if a == 5{
        second_code();

    }else if a == 11 {
        mc_vs_random();

    }else if a == 12 {
        mc_vs_random_2();

    }else if a == 13 {
        mc_vs_mc();
        
    }
}