use rand::Rng;
use super::board::DiceMove;

#[derive(Clone, )]
pub struct McAgent {
    gamma: f64,
    key: Vec<Vec<u64>>,
    pi: Vec<Vec<usize>>,
    q: Vec<f64>,
    cnt: Vec<f64>,
    memory: Vec< (Vec<u64>, (usize, DiceMove, usize), usize) >,
}
impl McAgent {
    pub fn new() -> Self {
        Self { gamma: 0.9, key:Vec::new(), pi: Vec::new(), q: Vec::new(), cnt: Vec::new(), memory: Vec::new() }
    }
    pub fn get_action(&self, actions: Vec<(usize, DiceMove, usize)>) -> (usize, DiceMove, usize) { // dice_num, dice_action, attack
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..actions.len());
        return actions[random];
    }
    pub fn add(&mut self, state: Vec<u64>, action: (usize, DiceMove, usize), reward: usize) {
        let data = (state, action, reward);
        self.memory.push(data);
    }
    pub fn reset(&mut self) {
        self.memory.clear();
    }
    pub fn eval(&mut self) {
        let mut g: f64 = 0.0;
        for data in self.memory.iter().rev() {
            let (state, action, reward) = data;
            g = self.gamma * g + *reward as f64;
            if let Some(v) = self.key.iter().position(|f| f==state) {
                self.cnt[v] += 1.0;
                self.q[v] += (g - self.q[v]) / self.cnt[v];
            }else {
                self.key.push(state.to_vec());
                self.cnt.push(0.0);
                self.q.push(0.0);
            }
        }
        println!("{:?}", self.q);
    }
}