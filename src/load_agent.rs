use std::{collections::HashMap, path::Path, fs::{OpenOptions, self, File}, io::{BufReader, BufRead}, result};

use csv::Error;

pub fn load_agent() -> HashMap<u64,  HashMap<(u8, u8), f32> > {
    let mut pi: HashMap<u64, HashMap<(u8, u8), f32>> = HashMap::new();
    let input_dir = Path::new("agent_file");
    let file_name = "agent_set1.txt";
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

// pub fn load_agent() -> HashMap<u64,  HashMap<(u8, u8), f32> >  {
//     let mut pi: HashMap<u64, HashMap<(u8, u8), f32>> = HashMap::new();
//     let input_dir = Path::new("../data/agent_file");
//     let file_name = "agent_set1.csv";
//     let path = input_dir.join(file_name);
//     let file_path = match path.is_file() {
//         false => panic!("no agent_set file"),
//         true => path,
//     };
//     let file = fs::read_to_string(file_path).expect("faild to open file");
//     let mut rdr = csv::ReaderBuilder::new()
//     .has_headers(false).from_reader(file.as_bytes());
//     for result in rdr.records() {
//         let mut actions: HashMap<(u8, u8), f32> = HashMap::new();
//         let record = result.expect("msg");
//         println!("{:?}", record);
//         let key: u64 = record[0].parse().expect("could not parse u64");//u64
//         let length: usize = record[1].parse().expect("could not parse");//length
//         for i in 2..length+1 {
//             if i%3 !=2 {continue;}
//             let dice_key: u8 = record[i].parse().expect("could not parse u8 dice");
//             let action_key: u8 = record[i+1].parse().expect("could not parse u8 action");
//             let prob: f32 = record[i+2].parse().expect("could not parse u8 key");
//             actions.entry((dice_key, action_key)).or_insert(prob);
//         }
//         pi.entry(key).or_insert(actions);
//     }
//     return pi;
    
// }