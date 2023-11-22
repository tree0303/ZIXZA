pub mod board;
use crate::zixza::board::{getcoordinate, DiceMove};
use crate::zixza::{board::Board, dice::Dice};
pub mod dice;
use crate::zixza::dice::{Player, getsidenums, getrightnum};
pub mod montecarlo;
pub mod randomagent;
use rand::Rng;
use std::io;

use self::board::BoardState;

const REWARD: isize = 5;

pub struct Zixza {
    player: Player,
    board: Board,
    dices: Vec<Dice>,
}

impl Zixza {
    pub fn new() -> Self {
        let board = Board::new();
        Self { player: Player::P1, board: board, dices: (Vec::new())}
    }
    pub fn testset(&mut self) {
        let alive = 1;
        let dice1 = Dice::new(1, 6, 5, 3, alive);
        let dice2 = Dice::new(2, 2, 6, 3, alive);
        let dice3 = Dice::new(3, 5, 1, 3, alive);
        let dice4 = Dice::new(4, 2, 6, 3, alive);
        let dice5 = Dice::new(5, 1, 4, 2, alive);
        let dice6 = Dice::new(6, 2, 4, 6, alive);
        self.dices.push(dice1);
        self.dices.push(dice2);
        self.dices.push(dice3);
        self.dices.push(dice4);
        self.dices.push(dice5);
        self.dices.push(dice6);
        self.board.putdice([2, 0], Player::P1, 1);
        self.board.putdice([1, 1], Player::P1, 2);
        self.board.putdice([0, 2], Player::P1, 3);
        self.board.putdice([4, 6], Player::P2, 1);
        self.board.putdice([5, 5], Player::P2, 2);
        self.board.putdice([6, 4], Player::P2, 3);
        if self.player==Player::P1 {self.board.turnboard();}
    }
    #[allow(dead_code)]
    pub fn show(&self) {
        self.board.show();
    }
    pub fn reset(&mut self) {
        self.board = Board::new();
        self.dices = Vec::new();
    }
    #[allow(dead_code)]
    pub fn setup(&mut self) {
        let mut count = 0;
        println!("Player1 => a,  Player2 => b");
        self.board.show();
        let mut p2_dices = Vec::new();
        loop {
            let mut top1: Vec<usize> = Vec::new();
            let mut top2: Vec<usize> = Vec::new();
            let mut rng = rand::thread_rng();
            let num1 = rng.gen_range(1..=6);
            let num2 = rng.gen_range(1..=6);
            if num1 == num2 {continue;}
            println!("Player1 : {}, Player2 : {}", num1, num2);
            top1.push(num1);
            top2.push(num2);
            let mut player = if num1 > num2 {Player::P1} else {Player::P2};
            count += 1;
            if player==Player::P1 {
                let dice = gendice(player, count, num1);
                setdice(&mut self.board, player, count);
                self.dices.push(dice);

                player = if player==Player::P1 {Player::P2} else {Player::P1};
                let dice = gendice(player, count+3, num2);
                setdice(&mut self.board, player, count);
                p2_dices.push(dice);
            } else {
                let dice = gendice(player, count+3, num2);
                setdice(&mut self.board, player, count);
                p2_dices.push(dice);

                player = if player==Player::P1 {Player::P2} else {Player::P1};
                let dice = gendice(player, count, num1);
                setdice(&mut self.board, player, count);
                self.dices.push(dice);
            }
            if count == 3 {
                if top1.iter().sum::<usize>() == top2.iter().sum() {
                    self.player = if top1[0] > top2[0] {Player::P1} else {Player::P2};
                } else if top1.iter().sum::<usize>() > top2.iter().sum() {
                    self.player = Player::P1;
                } else { 
                    self.player = Player::P2;
                }
                if self.player==Player::P1 {self.board.turnboard();}
                break;
            }
        }
        self.dices.extend(p2_dices);

        fn gendice(player: Player, num: usize, top: usize) -> Dice {
            println!("{}", player.to_string());
            println!("Top_Number is {}",top);
            let left = loop {
                println!("Choose a left_number from{:?}", getsidenums(top));
                let input = input_usize(); // サイコロの向き
                if getsidenums(top).iter().any(|v| v==&input) {
                    break input;
                }
            };
            Dice::new(num as usize, top as usize, left as usize, getrightnum(top, left) as usize, 1)
        }
        fn setdice(board: &mut Board, player: Player, count: usize) {
            let mut list: Vec<[usize; 2]> = board.initplace(player);
            if list.len() != 1{
                print!("Choose a dice position from [");
                list = list.iter().enumerate().map(|(i, v)| { 
                    print!("{}: {}",i+1,getcoordinate(&v[0], &v[1]).as_str());
                    if i < list.len()-1 {print!(", ");}
                    *v
                }).collect();
                println!("]");
                let position = loop {
                    let input = input_usize(); // サイコロの初期位置
                    if input > 0 && input <= list.len() {
                        break list[input-1];
                    }
                };
                board.putdice(position, player, count);
            } else {
                let position = list[0];
                println!("dice position is {}", getcoordinate(&position[0], &position[1]));
                board.putdice(position, player, count);
            }
            
        }
    }
    #[allow(dead_code)]
    pub fn start(&mut self) {
        self.board.setboardstate(BoardState::InMatch);
        'main: loop {
            self.board.step_count();
            println!("");
            println!("turn_count {}", self.board.getsteps());
            println!("Player: {}",self.player.to_string());
            println!("dice [top,left right]");
            for v in &self.dices {
                v.show();
            }
            self.board.show();
            
            let (dice_num, dicemove, attack) = self.select_dice_move();
            match dicemove { //dice_num 123456
                DiceMove::ForwardLeft => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_left();
                    self.board.forward_left(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::ForwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_right();
                    self.board.forward_right(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::BackwardLeft => {
                    self.dices[dice_num-1].backward_left();
                    if attack != 6 {self.dices[attack].dead()};
                    self.board.backward_left(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::BackwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].backward_right();
                    self.board.backward_right(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::TurnLeft => {
                    self.dices[dice_num-1].turn_left();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::TurnRight => {
                    self.dices[dice_num-1].turn_right();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::BeforeMove => print!("err"),
                DiceMove::Path => self.board.sameboard_count(),
            }
            let board_check = self.boardcheck();
            match board_check.0 {
                BoardState::BeforeMatch => println!("err"),
                BoardState::InMatch => {
                    self.player = if self.player==Player::P1 {Player::P2} else {Player::P1};
                    self.board.turnboard();
                },
                BoardState::Finish => break 'main,
            }
        }
    }
    pub fn select_action(&mut self) -> (usize, DiceMove, usize){
        self.board.step_count();
        println!("");
        println!("turn_count {}", self.board.getsteps());
        println!("Player: {}",self.player.to_string());
        println!("dice [top,left right]");
        for v in &self.dices {
            v.show();
        }
        self.board.show();
        let action = self.select_dice_move();
        return action;
    }
    pub fn step(&mut self, action: (usize, DiceMove, usize)) -> (u64, isize, bool, usize){ // action(dice_num, dice_action, attack)  next_state, reward, done, how_to_win
        // for v in &self.dices {
        //     v.show();
        // }
        // self.board.show();
        self.board.setboardstate(BoardState::InMatch);
        let (dice_num, dice_action, attack) = (action.0, action.1, action.2);
        let mut done = false;
        let mut win = 0;
        if dice_num == 0{
            self.board.step_count();
            println!("");
            println!("turn_count {}", self.board.getsteps());
            println!("Player: {}",self.player.to_string());
            println!("dice [top,left right]");
            for v in &self.dices {
                v.show();
            }
            self.board.show();
            let (dice_num, dicemove, attack) = self.select_dice_move();
            match dicemove {
                DiceMove::ForwardLeft => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_left();
                    self.board.forward_left(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::ForwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_right();
                    self.board.forward_right(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::BackwardLeft => {
                    self.dices[dice_num-1].backward_left();
                    if attack != 6 {self.dices[attack].dead()};
                    self.board.backward_left(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::BackwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].backward_right();
                    self.board.backward_right(dice_num);
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::TurnLeft => {
                    self.dices[dice_num-1].turn_left();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::TurnRight => {
                    self.dices[dice_num-1].turn_right();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dice_num, dicemove)
                },
                DiceMove::BeforeMove => print!("err"),
                DiceMove::Path => self.board.sameboard_count(),
            }
            let board_check = self.boardcheck();
            match board_check.0 {
                BoardState::BeforeMatch => println!("err"),
                BoardState::InMatch => {
                    self.player = if self.player==Player::P1 {Player::P2} else {Player::P1};
                    self.board.turnboard();
                },
                BoardState::Finish => {
                    done = true;
                    win = board_check.1;
                },
            }
            return (0, 0, done, win);
        } else {
            let mut reward: isize = 0;
            match dice_action {
                DiceMove::ForwardLeft => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_left();
                    self.board.forward_left(dice_num);
                    self.board.rewind(self.player, dice_num, dice_action)
                },
                DiceMove::ForwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_right();
                    self.board.forward_right(dice_num);
                    self.board.rewind(self.player, dice_num, dice_action)
                },
                DiceMove::BackwardLeft => {
                    self.dices[dice_num-1].backward_left();
                    if attack != 6 {self.dices[attack].dead()};
                    self.board.backward_left(dice_num);
                    self.board.rewind(self.player, dice_num, dice_action)
                },
                DiceMove::BackwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].backward_right();
                    self.board.backward_right(dice_num);
                    self.board.rewind(self.player, dice_num, dice_action)
                },
                DiceMove::TurnLeft => {
                    self.dices[dice_num-1].turn_left();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dice_num, dice_action)
                },
                DiceMove::TurnRight => {
                    self.dices[dice_num-1].turn_right();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dice_num, dice_action)
                },
                DiceMove::BeforeMove => print!("err"),
                DiceMove::Path => self.board.sameboard_count(),
            }
            let board_check = self.boardcheck();
            match board_check.0 {
                BoardState::BeforeMatch => println!("err"),
                BoardState::InMatch => {
                    self.player = if self.player==Player::P1 {Player::P2} else {Player::P1};
                    self.board.turnboard();
                },
                BoardState::Finish => {
                    reward = REWARD;
                    done = true;
                    win = board_check.1;
                },
            }
            let next_state = self.get_state();
            return (next_state, reward, done, win);
        }
    }

    pub fn get_state(&mut self) -> u64 {
        let mut buf_state: Vec<u64> = Vec::new();
        if self.player == Player::P1 {
            buf_state.push(1);
        }else {
            buf_state.push(0);
        }
        for (n, dice) in self.dices.iter().enumerate() {
            let buf = self.board.get_dice_position(n+1);
            let dice_position = to_binary(change_48to31(buf[0]*7+buf[1]));
            let dice_info = to_binary(change_diceinfo_31(dice.gettop(), dice.getleft()));
            buf_state.extend(dice_position);
            buf_state.extend(dice_info);
        }
        let mut m = "100".to_string();
        let str_state = buf_state.iter().map(|v| v.to_string()).collect::<Vec<String>>().concat();
        m.push_str(&str_state);
        let state = u64::from_str_radix(&m, 2).unwrap();
        return state;
    }
    
    pub fn get_actions(&mut self)-> Vec<(usize, DiceMove, usize)> { // action(dice_num, dice_action, attack)
        let mut actions: Vec<(usize, DiceMove, usize)> = Vec::new();
        let (dice1, dice2) = self.dices.split_at(3);
        let player_dice: Vec<Dice> = match self.player {
            Player::P1 => dice1.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
            Player::P2 => dice2.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
        };
        let player_dice_nums: Vec<usize> = player_dice.iter().map(|v| v.getnum()).collect(); // 123456
        for dice_num in player_dice_nums {
            let actionslist = self.board.dice_move(dice_num, &self.dices); // attack 012345 non6
            for action in actionslist {
                let data = (dice_num, action.0, action.1);
                actions.push(data);
            }
        }
        return actions;
    }

    pub fn boardcheck(&mut self) -> (BoardState, usize){
        let mut how_win = 0;
        if self.board.getsameboardcount() == 3 {//1->引き分け
            self.board.setboardstate(BoardState::Finish);
            how_win = 1;
            // println!("draw");//TODO draw
        }
        how_win = self.board.win_check(self.player); //2->占拠，3->到達
        let (dice1, dice2) = self.dices.split_at(3);
        let enemy_player = if self.player==Player::P1 {Player::P2} else {Player::P1};
        let alive_dice: Vec<Dice> = match enemy_player {
            Player::P1 => dice1.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
            Player::P2 => dice2.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
        };
        if alive_dice.iter().len() <= 1 {
            self.board.setboardstate(BoardState::Finish);
            how_win = 4;//4->攻撃
        }
        (self.board.getboardstate(), how_win)
    }
    fn select_dice_move(&mut self) -> (usize, DiceMove, usize){
        let (dice1, dice2) = self.dices.split_at(3);
        let player_dice: Vec<Dice> = match self.player {
            Player::P1 => dice1.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
            Player::P2 => dice2.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
        };
        let player_dice_nums: Vec<usize> = player_dice.iter().map(|v| v.getnum()).collect();
        println!("Choose moving dice from {:?}", player_dice_nums);
        let (dice_num, dicemove, attack) = 'select_dice: loop {
            let dice_num = input_usize(); // 動かすサイコロの選択
            if player_dice_nums.iter().any(|v| *v==dice_num) {
                loop {
                    let actions = self.board.dice_move(dice_num, &self.dices);
                    println!("Choose a movement from");
                    for (i, v) in actions.iter().map(|f| f.0).enumerate() {
                        println!("{}: {}", i+1, v.to_string());
                    }
                    let input = input_usize(); // 動かし方の選択
                    if input > 0 && input <= actions.len() {
                        break 'select_dice (dice_num, actions[input-1].0, actions[input-1].1);
                    }
                    println!("again");
                }
            }
        };
        return (dice_num, dicemove, attack);
    }
    
}

pub fn input_usize() -> usize {
    let mut input = String::new();
    'input: loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<usize>() {
                    Ok(n) => break n,
                    Err(_) => {
                        println!("again");
                        input = "".to_string();
                        continue 'input;
                    }
                }
            }
            Err(_) => {
                println!("again");
                input = "".to_string();
                continue 'input;
            }
        };
    }
    
}

fn change_48to31(position: usize) -> usize{
    match position {
        2 => 0,
        3 => 1,
        8 => 2,
        9 => 3,
        10 => 4,
        11 => 5,
        14 => 6,
        15 => 7,
        16 => 8,
        17 => 9,
        18 => 10,
        19 => 11,
        21 => 12,
        22 => 13,
        23 => 14,
        24 => 15,
        25 => 16,
        26 => 17,
        27 => 18,
        29 => 19,
        30 => 20,
        31 => 21,
        32 => 22,
        33 => 23,
        34 => 24,
        37 => 25,
        38 => 26,
        39 => 27,
        40 => 28,
        45 => 29,
        46 => 30,
        _ => 31,
    }
}

fn change_diceinfo_31(top: usize, left: usize) -> usize{
    match top {
        1 => match left {
            2 => 0,
            3 => 1,
            4 => 2,
            5 => 3,
            _ => 24, //error
        },
        2 => match left {
            1 => 4,
            3 => 5,
            4 => 6,
            6 => 7,
            _ => 24, //error
        },
        3 => match left {
            1 => 8,
            2 => 9,
            5 => 10,
            6 => 11,
            _ => 24, //error
        },
        4 => match left {
            1 => 12,
            2 => 13,
            5 => 14,
            6 => 15,
            _ => 24, //error
        },
        5 => match left {
            1 => 16,
            3 => 17,
            4 => 18,
            6 => 19,
            _ => 24, //error
        },
        6 => match left {
            2 => 20,
            3 => 21,
            4 => 22,
            5 => 23,
            _ => 24, //error
        },
        _ => 0 //error
    }
}

pub fn to_binary(num: usize) -> Vec<u64> {
    let mut data: Vec<u64> = Vec::new();
    let mut n = num as u64;
    match num < 32 {
        true => {
            for _ in 0..5 {
                if n > 1 {
                    data.insert(0, n%2);
                }else {
                    data.insert(0, 0);
                }
                n /= 2;
            }
        }
        false => {
            for _ in 0..num.ilog2(){
                data.insert(0, n%2);
                n /= 2;
            }
        }
    }
    return data;
}
