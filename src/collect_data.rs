
use std::{path::Path, fs::{create_dir_all, OpenOptions}, io::{BufWriter, Write}, collections::HashMap};
// ゲームの保存
pub fn write_data(memories: &Vec< (u64, (u8, u8, u8), i8, Vec<(u8, u8, u8)>) >) {//state, action, reward, actions) 
    let output_dir = Path::new("../data_new/data_file");
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
        let mut str_buf = format!("{},{},{},{},{}",a,b,c,d,e);
        for (f, g, h) in actions {
            let str = format!(",{},{},{}",f,g,h);
            str_buf.push_str(&str);
        }
        writeln!(w, "{}", str_buf).unwrap();
    }
}
// エージェントの保存
pub fn save_agent(pi :&HashMap<u64,  HashMap<(u8, u8), f32>  >) {// state, (dice, movement), 確率
    let output_dir = Path::new("../data_new/agent_file");
    create_dir_all(&output_dir).unwrap();
    let mut number = 0;
    let file_path = loop {
        number+=1;
        let file_name = format!("agent_set{}.txt",number);
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
    for (state, actions) in pi {
        let length = actions.len();
        let mut str_buf = format!("{},{}",state,length);
        for ((d,d_move),prob) in actions {
            let str = format!(",{},{},{}",d,d_move,prob);
            str_buf.push_str(&str);
        }
        // state, actions_size, actions<dice, move, prob>
        writeln!(w, "{}", str_buf).unwrap();
        // println!("{}",str_buf);
    }
} 
// 対戦結果の保存
pub fn mc_vs_random_data(filename: &str, buf: Vec<(bool, bool, &str, usize)>) {//first, mc_win, how, steps
    println!("write");
    let model_num = filename.chars().filter_map(|f| f.to_string().parse().ok()).collect::<Vec<u32>>()[0];
    let output_dir = Path::new("../data_new/mc_vs_random");
    create_dir_all(&output_dir).unwrap();
    let mut number = 0;
    let file_path = loop {
        number+=1;
        let file_name = format!("vs{}_data{}.csv",model_num,number);
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
    writeln!(w,"mc_win,how,steps").unwrap();
    for (first, mc_win, how, steps) in buf {
        let str = if first {"first"}else {"second"};
        let mut str_buf = format!("{},{},{}",mc_win, how, steps);
        writeln!(w, "{}", str_buf).unwrap();
    }
}
// 対戦結果の保存
pub fn mc_vs_random_data_2(filename: &str,mc_win: bool, how: &str, steps: usize) {//first, mc_win, how, steps
    println!("write");
    let model_num = filename.chars().filter_map(|f| f.to_string().parse().ok()).collect::<Vec<u32>>()[0];
    let output_dir = Path::new("../data_new/mc_vs_random_ex");
    create_dir_all(&output_dir).unwrap();
    let mut number = 0;
    let file_path = loop {
        number+=1;
        let file_name = format!("vs{}_data_exx.csv",model_num);
        let path = output_dir.join(file_name);
        // if path.is_file() {
        //     break path;
        // }else {
        //     continue;
        // }
        break path;
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
    // writeln!(w,"mc_win,how,steps").unwrap();
    let mut str_buf = format!("{},{},{}",mc_win, how, steps);
    writeln!(w, "{}", str_buf).unwrap();
}
// 対戦結果の保存
pub fn mc_vs_random_data_ex(filename: &str,index:usize , first: bool,mc_win: bool, how: &str, steps: usize) {//first, mc_win, how, steps
    println!("write");
    let model_num = filename.chars().filter_map(|f| f.to_string().parse().ok()).collect::<Vec<u32>>()[0];
    let output_dir = Path::new("../data_new/mc_vs_mc");
    create_dir_all(&output_dir).unwrap();
    let mut number = 0;
    let file_path = loop {
        number+=1;
        let file_name = format!("{}_{}.csv",filename,number);
        let path = output_dir.join(file_name);
        break path;
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
    let str_buf = format!("{},{},{},{}",first, mc_win, how, steps);
    writeln!(w, "{}", str_buf).unwrap();
}
// 対戦結果の保存
pub fn mc_vs_mc_data(filename: &str,buf: Vec<(bool, &str, usize)>) {//mc_win, how, steps
    println!("write");
    // let model_num = filename.chars().filter_map(|f| f.to_string().parse().ok()).collect::<Vec<u32>>()[0];
    let output_dir = Path::new("../data_new/mc_vs_mc");
    create_dir_all(&output_dir).unwrap();
    let mut number = 0;
    let file_path = loop {
        number+=1;
        let file_name = format!("{}_{}.csv",filename,number);
        let path = output_dir.join(file_name);
        // if path.is_file() {
        //     break path;
        // }else {
        //     continue;
        // }
        break path;
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
    for (mc_win, how, steps) in buf {
        let mut str_buf = format!("{},{},{}",mc_win, how, steps);
        writeln!(w, "{}", str_buf).unwrap();
    }
}