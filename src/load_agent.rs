use std::{collections::HashMap, path::Path, fs::{OpenOptions, self, File}, io::{BufReader, BufRead}, result};

use csv::Error;

pub fn load_agent() -> HashMap<u64,  HashMap<(u8, u8), f32> > {
    println!("load_agent");
    let mut pi: HashMap<u64, HashMap<(u8, u8), f32>> = HashMap::new();
    let input_dir = Path::new("../data_new/agent_file");
    let file_name = "agent_set6.txt";
    let path = input_dir.join(file_name);
    let file_path = match path.is_file() {
        false => panic!("no agent_set file"),
        true => path,
    };
    let file = match File::open(file_path) {
        Err(e) => panic!("faild to open file"),
        Ok(v) => v,
    };
    let reader = BufReader::new(file);
    for result in reader.lines(){
        let line = result.expect("msg");
        let mut vec = Vec::new();
        vec = line.split(',').collect();
        let mut actions: HashMap<(u8, u8), f32> = HashMap::new();
        for (i, _) in vec.iter().enumerate() {
            if i%3 != 2{ continue;}
            actions.entry((vec[i].parse().expect("a"),vec[i+1].parse().expect("a"))).or_insert(vec[i+2].parse().expect("a"));
        }
        pi.entry(vec[0].parse().expect("a")).or_insert(actions);
    }
    return pi;
}

pub fn load_agent_into_filename(file_name:&str) -> HashMap<u64,  HashMap<(u8, u8), f32> > {
    println!("load_agent  {}", file_name);
    let mut pi: HashMap<u64, HashMap<(u8, u8), f32>> = HashMap::new();
    let input_dir = Path::new("../data_new/agent_file");
    // let file_name = "agent_set6.txt";
    let path = input_dir.join(file_name);
    let file_path = match path.is_file() {
        false => panic!("no agent_set file"),
        true => path,
    };
    let file = match File::open(file_path) {
        Err(e) => panic!("faild to open file"),
        Ok(v) => v,
    };
    let reader = BufReader::new(file);
    for result in reader.lines(){
        let line = result.expect("msg");
        let mut vec = Vec::new();
        vec = line.split(',').collect();
        let mut actions: HashMap<(u8, u8), f32> = HashMap::new();
        for (i, _) in vec.iter().enumerate() {
            if i%3 != 2{ continue;}
            actions.entry((vec[i].parse().expect("a"),vec[i+1].parse().expect("a"))).or_insert(vec[i+2].parse().expect("a"));
        }
        pi.entry(vec[0].parse().expect("a")).or_insert(actions);
    }
    return pi;
}
