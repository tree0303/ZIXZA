use std::collections::HashMap;
use crate::zixza::to_binary;
use rand::{distributions::WeightedIndex, prelude::Distribution};
use super::board::DiceMove;
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct ActionKey(usize, DiceMove, usize);

pub struct McAgent {
    gamma: f32,
    epsolon: f32,
    alpha: f32,
    q: HashMap<(u64, (u8, u8, u8)), f32>, // state, action, 評価
    memory: Vec< (u64, (u8, u8, u8), i8, Vec<(u8, u8, u8)>) >,//state, action, reward, actions
    pi: HashMap<u64,  HashMap<(u8, u8), f32>  >, // state, (dice, movement), 確率
}
impl McAgent {
    pub fn new() -> Self {
        Self { gamma: 0.9, epsolon: 0.3, alpha: 0.1, q: HashMap::new(), memory: Vec::new(), pi: HashMap::new() }
    }
    pub fn get_action(&mut self, state: &u64, actions: &Vec<(usize, DiceMove, usize)>) -> (usize, DiceMove, usize) { // dice_num, dice_action, attack
        if actions.len() == 0{
            return (7, DiceMove::Path, 6);
        }
        let actions_keys: Vec<(u8, u8/*dicemove */)> = actions.iter().map(|&(x, y, _)| (x as u8, y.to_u8())).collect();
        let action_probs: HashMap<(u8, u8), f32> = match self.pi.get_mut(state) {
            Some(v) => {
                actions_keys.iter().map(|&k| {
                    let key = (k.0 as u8, k.1);
                    let value = match v.get(&key) {
                        Some(x) => *x,
                        None => {
                            v.entry(key).or_insert(1.0 / 18.0);
                            1.0 / 18.0
                        },
                    };
                    (k, value)
                }).collect()
            },
            None => {
                let mut buf: HashMap<(u8, u8/*dicemove */), f32> = HashMap::new();
                let buff: HashMap<(u8, u8/*dicemove */), f32> = actions_keys.iter().map(|&k| {
                    buf.insert(k, 1.0 / 18.0);
                    (k, 1.0 / 18.0)
                }).collect();
                self.pi.insert(*state, buf);
                buff
            }
        };
        let actions_vec: Vec<(u8, u8/*dicemove */)> = action_probs.keys().cloned().collect::<Vec<(u8, u8/*dicemove */)>>();
        let probs: Vec<f32> = action_probs.values().cloned().collect::<Vec<f32>>();
        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(probs).unwrap();
        let choice = actions_vec[dist.sample(&mut rng)];
        //ここおかしい
        let action =  actions.iter().find(|(x, y, _)| {
            (*x, *y) == choice
        }).unwrap();
        return *action;
    }

    pub fn add(&mut self, state: u64, action: (usize, DiceMove, usize), reward: isize, actions: Vec<(usize, DiceMove, usize)>) {
        let ch_actions: Vec<(u8, u8, u8)> = actions.iter().map(|v| (v.0 as u8, v.1.to_u8(), v.2 as u8)).collect();
        let data: (u64, (u8, u8, u8), i8, Vec<(u8, u8, u8)>) = (state, (action.0 as u8, action.1.to_u8(), action.2 as u8), reward as i8, ch_actions);
        self.memory.push(data);
    }
    pub fn reset(&mut self) {
        self.memory.clear();
    }
    
    pub fn update(&mut self) {
        let mut g = 0.0;
        let mut count = -1;
        for data in self.memory.iter().rev() {
            count*=-1;
            if count == -1 {continue;}
            let (state, action, reward, actions) = data;
            g = self.gamma * g + *reward as f32;
            let key = (*state, *action);
            self.q.entry(key).or_insert(0.0);
            self.q.insert(key, self.q.get(&key).cloned().unwrap_or(0.0) + (g - self.q[&key]) * self.alpha);
            self.pi.insert(*state, greedy_probs(&self.q, *state, actions, self.epsolon));
        }
    }
    pub fn q_show(&self, max_count: usize) {
        let mut count = 0;
        for (qq, dd) in self.q.iter() {
            count+=1;
            let a = to_binary(qq.0 as usize).iter().map(|v| v.to_string()).collect::<Vec<String>>().concat();
            println!("state:  {:?},   value   {}", a, dd);
            // println!("{}",qq.0);
            if count==max_count{break;}
        }
        let po = match self.q.iter().find(|((c, _), _)| *c ==10456235733654307266) {
            Some(v) => *v.1,
            None => 0.0,
        };
        let gd = match self.q.iter().find(|((c, _), _)| *c ==10700626897810751490) {
            Some(v) => *v.1,
            None => 0.0,
        };
        // println!("{}", po);
        println!("q {}",self.q.len());
        println!("pi{}", self.pi.len());
    }
}
pub fn greedy_probs(q: &HashMap<(u64, (u8, u8, u8)), f32>, state: u64, actions: &Vec<(u8, u8, u8)>, epsilon: f32) -> HashMap<(u8, u8), f32>{
    let mut qs = HashMap::new();
    let action_size = actions.len();
    for action in actions {
        let key = (state, *action);
        if let Some(v) = q.get(&key) {
            qs.insert(action, v);
        }
    }
    let max_num = qs.iter().fold(0.0, |m, (_, &fv)| fv.max(m));
    let max_action = qs.iter().find(|(&k, &v)| *v == max_num);
    
    let base_prob = epsilon / action_size as f32;
    let mut action_probs = HashMap::new();
    for action in actions.iter() {
        action_probs.insert((action.0, action.1), base_prob);
    }
    match max_action {
        Some((v, _)) => {*action_probs.entry( (v.0, v.1)).or_insert(0.0) += 1.0 - epsilon;},
        None => println!("errer_max_action"),
    };
    
    return action_probs;
}
/*
0 ForwardLeft
1 ForwardRight
2 BackwardLeft
3 BackwardRight
4 TurnLeft
5 TurnRight
 */