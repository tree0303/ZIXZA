<<<<<<< HEAD

use std::{path::Path, fs::{create_dir_all, OpenOptions}, io::{BufWriter, Write}, collections::HashMap};

pub fn write_data(memories: &Vec< (u64, (u8, u8, u8), i8, Vec<(u8, u8, u8)>) >) {//state, action, reward, actions) 
    let output_dir = Path::new("data_file");
    create_dir_all(&output_dir).unwrap();
    let mut number = 0;
    let file_path = loop {
        number+=1;
        let file_name = format!("data_set{}.csv",number);
        let path = output_dir.join(file_name);
        if path.is_file() {
            continue;
        }else {
            break path;
        }
    };
    let file = match OpenOptions::new()
    .append(true)
    .write(true)
    .truncate(false)
    .create(true)
    .open(file_path) {
        Err(e) => panic!("could not create : {}", e),
        Ok(v) => v,
    };
    let mut w = BufWriter::new(file);
    for (a,(b, c, d),e, actions) in memories {
        let mut str_buf = format!("{},{},{},{},{},",a,b,c,d,e);
        for (f, g, h) in actions {
            let str = format!("{},{},{}",f,g,h);
            str_buf.push_str(&str);
        }
        writeln!(w, "{}", str_buf).unwrap();
    }
}

pub fn save_agemt(pi :&HashMap<u64,  HashMap<(u8, u8), f32>  >) {// state, (dice, movement), 確率
    let output_dir = Path::new("agent_file");
    create_dir_all(&output_dir).unwrap();
    let mut number = 0;
    let file_path = loop {
        number+=1;
        let file_name = format!("agent_set{}.csv",number);
        let path = output_dir.join(file_name);
        if path.is_file() {
            continue;
        }else {
            break path;
        }
    };
    let file = match OpenOptions::new()
    .append(true)
    .write(true)
    .truncate(false)
    .create(true)
    .open(file_path) {
        Err(e) => panic!("could not create : {}", e),
        Ok(v) => v,
    };
    let mut w = BufWriter::new(file);
    for (a, actions) in pi {
        let mut str_buf = format!("{},,",a);
        for ((b,c),d) in actions {
            let str = format!("{},{},{}",b,c,d);
            str_buf.push_str(&str);
        }
        writeln!(w, "{}", str_buf).unwrap();
    }
} 
=======
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
>>>>>>> origin/master
