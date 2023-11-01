use std::{path::Path, fs::{create_dir_all, File}, io::{BufWriter, Write}, collections::HashMap};
use crate::zixza::board::DiceMove;

pub fn write_data(data: Vec<(u64, usize, usize)>) { //board_state: u64, turns: usize, how_win: usize
    let output_dir = Path::new("data_file");
    create_dir_all(&output_dir).unwrap();
    let file_path = output_dir.join("data_set.csv");
    // board
    // turns 奇数→先手勝 偶数→後手勝
    // how to win
    // 
    let file = match File::create(file_path) {
        Err(e) => panic!("could not create : {}",e),
        Ok(v) => v,
    };
    let mut w = BufWriter::new(file);
    for (board_state, turns, how_win) in data {
        let str_buf = format!("{},{},{}", board_state, turns, howwin_to_string(how_win));
        writeln!(w, "{}",str_buf).unwrap();
    }
}

pub fn save_q(data: HashMap<(u64, (usize, DiceMove, usize)), f32>) {
    let output_dir = Path::new("data_file");
    create_dir_all(&output_dir).unwrap();
    let file_path = output_dir.join("agent_data.csv");
    let file = match File::create(file_path) {
        Err(e) => panic!("could not create : {}",e),
        Ok(v) => v,
    };
    let mut w = BufWriter::new(file);
    for (key, value) in data {
        let (state, (dice_num, action, attack_flag)) = key;
        let str_buf = format!("{},{},{},{},{}", state, dice_num, action.to_string(), attack_flag, value);
        writeln!(w, "{}",str_buf).unwrap();
    }
}




fn howwin_to_string(how_win: usize) -> String{
    let how_win_str = match how_win {
        1 => String::from("draw"),
        2 => String::from("occupation"),
        3 => String::from("reach"),
        4 => String::from("attack"),
        _ => String::from("inmatch"),
    };
    how_win_str
}