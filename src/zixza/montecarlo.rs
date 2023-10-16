use std::collections::HashMap;

use rand::Rng;
use super::board::DiceMove;
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct DataKey(u64, (usize, DiceMove, usize));

pub struct McAgent {
    gamma: f64,
    // key: Vec<Vec<u64>>,
    // pi: Vec<Vec<usize>>,
    // q: Vec<f64>,
    // cnt: Vec<f64>,
    // memory: Vec< (Vec<u64>, (usize, DiceMove, usize), usize) >,
    q: HashMap<DataKey, f64>,
    cnts: HashMap<DataKey, f64>,
    memory: Vec< (u64, (usize, DiceMove, usize), usize) >,
    pi: HashMap<DataKey, f64>,
}
impl McAgent {
    pub fn new() -> Self {
        Self { gamma: 0.9, q: HashMap::new(), cnts: HashMap::new(), memory: Vec::new(), pi: HashMap::new() }
    }
    pub fn get_action(&self, actions: Vec<(usize, DiceMove, usize)>) -> (usize, DiceMove, usize) { // dice_num, dice_action, attack
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..actions.len());
        return actions[random];
    }
    pub fn add(&mut self, vec_state: Vec<u64>, action: (usize, DiceMove, usize), reward: usize) {
        let mut m = "100".to_string();
        let str_state = vec_state.iter().map(|v| v.to_string()).collect::<Vec<String>>().concat();
        m.push_str(&str_state);
        let state = u64::from_str_radix(&m, 2).unwrap();
        println!("{:?}", m);
        let data = (state, action, reward);
        self.memory.push(data);
    }
    pub fn reset(&mut self) {
        self.memory.clear();
    }
    // pub fn eval(&mut self) {
    //     let mut g: f64 = 0.0;
    //     for data in self.memory.iter().rev() {
    //         let (state, action, reward) = data;
    //         g = self.gamma * g + *reward as f64;
    //         *self.cnt.entry(*state).or_insert(0.0) += 1.0;
    //         *self.q.entry(*state).or_insert(0.0);
    //         *self.q.entry(*state).or_insert(0.0) += (g - self.q.get(state).unwrap()) / self.cnt.get(state).unwrap();
    //     }
    //     // println!("{:?}", self.q.keys());
    // }
    pub fn update(&mut self) {
        let mut g = 0.0;
        for data in self.memory.iter().rev() {
            let (state, action, reward) = *data;
            g = self.gamma * g + reward as f64;
            let key: DataKey = DataKey(state, action);
            *self.cnts.entry(key).or_insert(0.0) += 1.0;
            let q_value = self.q.entry(key).or_insert(0.0);
            *q_value += (g - *q_value) / self.cnts[&key];
            // self.pi.insert(state, greedy_probs(&self.q, state));
        }
    }
    
}
// pub fn greedy_probs(q: HashMap<u64, f64>, state: u64, epsilon: f64, action_size: usize) {
//     let mut qs = Vec::new();
//     for action in 0..action_size {
//         let q_value = q.get(k)
//     }
// }