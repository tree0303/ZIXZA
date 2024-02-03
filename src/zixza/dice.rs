
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Player {
    P1 = 1,
    P2 = 2,
}
impl Player {
    pub fn to_string(&self) -> &str{
        match self {
            Player::P1 => return "Player1",
            Player::P2 => return "Player2",
        };
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Dice {
    num: usize,//123456が入る
    top: usize,
    left: usize,//左手前
    right: usize,//右手前
    alive: usize, //0 dead or 1 live
}

impl Dice {
    // 初期インスタンス
    pub fn new(num: usize, top: usize, left: usize, right: usize, alive: usize) -> Self{
        Self { num: (num), top: (top), left: (left), right: (right), alive: (alive) }
    }
    // ターミナル表示
    pub fn show(&self) {
        println!("dice {}: [{}, {}, {}]", self.num, self.top, self.left, self.right);
    }
    // 左前移動
    pub fn forward_left(&mut self) {
        let right = self.right;
        self.right = getbacknum(self.top);
        self.top = right;
    }
    // 右前移動
    pub fn forward_right(&mut self) {
        let left = self.left;
        self.left = getbacknum(self.top);
        self.top = left;
    }
    // 左後ろ移動
    pub fn backward_left(&mut self) {
        let top = self.top;
        self.top = getbacknum(self.left);
        self.left = top;
    }
    // 右後ろ移動
    pub fn backward_right(&mut self) {
        let top = self.top;
        self.top = getbacknum(self.right);
        self.right = top;
    }
    // 左回転
    pub fn turn_left(&mut self) {
        let left = self.left;
        self.left = getbacknum(self.right);
        self.right = left;
    }
    // 右回転
    pub fn turn_right(&mut self) {
        let right = self.right;
        self.right = getbacknum(self.left);
        self.left = right;
    }
    pub fn dead(&mut self) {
        self.alive=0;
    }
    pub fn getalive(&self) -> usize {
        self.alive
    }
    pub fn gettop(&self) -> usize {
        self.top
    }
    pub fn getleft(&self) -> usize {
        self.left
    }
    pub fn getnum(&self) -> usize {
        self.num
    }
    #[allow(dead_code)]
    pub fn error_dice(&self) -> Vec<usize>{
        let mut s = Vec::new();
        s.push(self.num);
        if self.left==self.top {
            println!("left_top");
            s.push(1);
        }
        if self.left==self.right {
            println!("left_right");
            s.push(2);
        }
        if self.top==self.right {
            println!("top_right");
            s.push(3);
        }
        return s;
    }
    
}
// 上と左手前の数字から右手前の数字を得る
pub fn getrightnum(topnum: usize, leftnum: usize) -> usize {
    match topnum {
        1 => match leftnum {
            2 => 3,
            3 => 5,
            4 => 2,
            5 => 4,
            _ => 0, //error
        },
        2 => match leftnum {
            1 => 4,
            3 => 1,
            4 => 6,
            6 => 3,
            _ => 0, //error
        },
        3 => match leftnum {
            1 => 2,
            2 => 6,
            5 => 1,
            6 => 5,
            _ => 0, //error
        },
        4 => match leftnum {
            1 => 5,
            2 => 1,
            5 => 6,
            6 => 2,
            _ => 0, //error
        },
        5 => match leftnum {
            1 => 3,
            3 => 6,
            4 => 1,
            6 => 4,
            _ => 0, //error
        },
        6 => match leftnum {
            2 => 4,
            3 => 2,
            4 => 5,
            5 => 3,
            _ => 0, //error
        },
        _ => 0 //error
    }
}
// 上の数字から側面の数字を得る
pub fn getsidenums(topnum: usize) -> Vec<usize> {
    let mut sidenums: Vec<usize> = Vec::new();
    for n in 1..=6{
        if n == topnum || n == getbacknum(topnum){continue;}
        sidenums.push(n);
    }
    return sidenums;
} 
// 反対の数字を得る
pub fn getbacknum(num: usize) -> usize {
    return 7 - num;
}

