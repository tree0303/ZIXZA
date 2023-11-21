
use std::{path::Path, fs::{create_dir_all, OpenOptions}, io::{BufWriter, Write}, collections::HashMap};

pub fn write_data(memories: &Vec< (u64, (u8, u8, u8), i8, Vec<(u8, u8, u8)>) >) {//state, action, reward, actions) 
    let output_dir = Path::new("../data/data_file");
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

pub fn save_agemt(pi :&HashMap<u64,  HashMap<(u8, u8), f32>  >) {// state, (dice, movement), 確率
    let output_dir = Path::new("../data/agent_file");
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
        let length = actions.len();
        let mut str_buf = format!("{},{}",a,length);
        for ((b,c),d) in actions {
            let str = format!(",{},{},{}",b,c,d);
            str_buf.push_str(&str);
        }
        writeln!(w, "{}", str_buf).unwrap();
        println!("{}",str_buf);
    }
} 