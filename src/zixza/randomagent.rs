use std::collections::HashMap;

use rand::Rng;
use crate::zixza::to_binary;

use super::board::DiceMove;

pub struct RandomAgent {
    gamma: f64,
    q: HashMap<u64, f64>,
    cnt: HashMap<u64, f64>,
    memory: Vec< (u64, (usize, DiceMove, usize), isize) >,
}
impl RandomAgent {
    pub fn new() -> Self {
        Self { gamma: 0.9, q: HashMap::new(), cnt: HashMap::new(), memory: Vec::new() }
    }
    pub fn get_action(&self, actions: Vec<(usize, DiceMove, usize)>) -> (usize, DiceMove, usize) { // dice_num, dice_action, attack
        let mut rng = rand::thread_rng();
        if actions.len()!=0{
            let random = rng.gen_range(0..actions.len());
            return actions[random];
        }
        return (7, DiceMove::Path, 6);
    }
    pub fn add(&mut self, state: u64, action: (usize, DiceMove, usize), reward: isize) {
        let data = (state, action, reward);
        self.memory.push(data);
    }
    pub fn reset(&mut self) {
        self.memory.clear();
    }
    pub fn eval(&mut self) {
        let mut g: f64 = 0.0;
        for data in self.memory.iter().rev() {
            let (state, _, reward) = data;
            g = self.gamma * g + *reward as f64;
            *self.cnt.entry(*state).or_insert(0.0) += 1.0;
            let _ = *self.q.entry(*state).or_insert(0.0);
            *self.q.entry(*state).or_insert(0.0) += (g - self.q.get(state).unwrap()) / self.cnt.get(state).unwrap();
        }
    }
    pub fn q_show(&self) {
        let mut count = 0;
        for (qq, dd) in self.q.iter() {
            count+=1;
            let a = to_binary(*qq as usize).iter().map(|v| v.to_string()).collect::<Vec<String>>().concat();
            println!("key:  {:?},   value   {}", a, dd);
            if count==10{break;}
        }
        println!("{}",self.q.len());
    }
    
}
