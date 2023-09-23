pub mod board;
use crate::zixza::board::{getcoordinate, DiceMove};
use crate::zixza::{board::Board, dice::Dice};
pub mod dice;
use crate::zixza::dice::{Player, getsidenums, getrightnum};
use rand::Rng;
use std::io;

use self::board::BoardState;
pub struct Zixza {
    player: Player,
    board: Board,
    dices: Vec<Dice>,
}

impl Zixza {
    pub fn new() -> Self {
        let board = Board::new();
        // Self { player: (Player::P1), board: (board) }  
        Self { player: Player::P1, board: board, dices: (Vec::new()) }
    }

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
                let input = input_usize();
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
                let input = input_usize();
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
            let (dice1, dice2) = self.dices.split_at(3);
            let player_dice: Vec<Dice> = match self.player {
                Player::P1 => dice1.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
                Player::P2 => dice2.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
            };
            let player_dice_nums: Vec<usize> = player_dice.iter().map(|v| v.getnum()).collect();
            println!("Choose moving dice from {:?}", player_dice_nums);
            let (dice_num, dicemove, attack) = 'select_dice: loop {
                let dice_num = input_usize();
                if player_dice_nums.iter().any(|v| *v==dice_num) {
                    loop {
                        let (dicemoves, attack) = self.board.dice_move(dice_num, &self.dices);
                        println!("Choose a movement from");
                        for (i, v) in dicemoves.iter().enumerate() {
                            println!("{}: {}", i+1, v.to_string());
                        }
                        let input = input_usize();
                        if input > 0 && input <= dicemoves.len() {
                            break 'select_dice (dice_num, dicemoves[input-1], attack);
                        }
                        println!("again");
                    }
                }
            };

            match dicemove {
                DiceMove::ForwardLeft => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_left();
                    self.board.forward_left(dice_num);
                    self.board.rewind(self.player, dicemove)
                },
                DiceMove::ForwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].forward_right();
                    self.board.forward_right(dice_num);
                    self.board.rewind(self.player, dicemove)
                },
                DiceMove::BackwardLeft => {
                    self.dices[dice_num-1].backward_left();
                    if attack != 6 {self.dices[attack].dead()};
                    self.board.backward_left(dice_num);
                    self.board.rewind(self.player, dicemove)
                },
                DiceMove::BackwardRight => {
                    if attack != 6 {self.dices[attack].dead()};
                    self.dices[dice_num-1].backward_right();
                    self.board.backward_right(dice_num);
                    self.board.rewind(self.player, dicemove)
                },
                DiceMove::TurnLeft => {
                    self.dices[dice_num-1].turn_left();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dicemove)
                },
                DiceMove::TurnRight => {
                    self.dices[dice_num-1].turn_right();
                    self.board.sameboard_count();
                    self.board.rewind(self.player, dicemove)
                },
                DiceMove::BeforeMove => print!("err"),
            }
            match self.boardcheck() {
                BoardState::BeforeMatch => println!("err"),
                BoardState::InMatch => {
                    self.player = if self.player==Player::P1 {Player::P2} else {Player::P1};
                    self.board.turnboard();
                },
                BoardState::Draw => {
                    println!("Draw");
                    break 'main;
                },
                BoardState::Occupation => {
                    break 'main;
                },
                BoardState::Reach => {
                    break 'main;
                },
                BoardState::Seizure => {
                    break 'main;
                },
            }
            
        }
    }
    pub fn boardcheck(&mut self) -> BoardState{
        if self.board.getsameboardcount() == 3 {
            self.board.setboardstate(BoardState::Draw);
        }
        self.board.win_check(self.player); //占拠，到達
        let (dice1, dice2) = self.dices.split_at(3);
        let enemy_player = if self.player==Player::P1 {Player::P2} else {Player::P1};
        let alive_dice: Vec<Dice> = match enemy_player {
            Player::P1 => dice1.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
            Player::P2 => dice2.to_vec().into_iter().filter(|&v| v.getalive()==1).collect(),
        };
        if alive_dice.iter().len() <= 1 {
            self.board.setboardstate(BoardState::Seizure);
        }
        self.board.getboardstate()
    }
}


fn input_usize() -> usize {
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



