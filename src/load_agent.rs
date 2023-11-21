use std::{collections::HashMap, path::Path, fs::{OpenOptions, self}, io::BufReader, result};

use csv::Error;


pub fn load_agent() -> Result< HashMap<u64,  HashMap<(u8, u8), f32>  >, Box<Error>  >{
    let mut pi: HashMap<u64, HashMap<(u8, u8), f32>> = HashMap::new();
    let input_dir = Path::new("../data/agent_file");
    let file_name = "agent_set1.csv";
    let path = input_dir.join(file_name);
    let file_path = match path.is_file() {
        false => panic!("no agent_set file"),
        true => path,
    };
    let file = fs::read_to_string(file_path).expect("faild to open file");
    let mut rdr = csv::ReaderBuilder::new()
    .has_headers(false).from_reader(file.as_bytes());
    for result in rdr.records() {
        let mut actions: HashMap<(u8, u8), f32> = HashMap::new();
        let record = result?;
        let key: u64 = record[0].parse().expect("could not parse u64");//u64
        let length: usize = record[1].parse().expect("could not parse");//length
        for i in 2..length+1 {
            if i%3 !=2 {continue;}
            let dice_key: u8 = record[i].parse().expect("could not parse u8 dice");
            let action_key: u8 = record[i+1].parse().expect("could not parse u8 action");
            let prob: f32 = record[i+2].parse().expect("could not parse u8 key");
            actions.entry((dice_key, action_key)).or_insert(prob);
        }
        pi.entry(key).or_insert(actions);
    }
    // return pi;
    Ok(pi)
}