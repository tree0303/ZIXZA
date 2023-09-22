pub mod board;
use crate::zixza::board::{getcoordinate, Dice_move};
use crate::zixza::{board::Board, dice::Dice};
pub mod dice;
use crate::zixza::dice::{Player, getbacknum, getsidenums, getrightnum};
use rand::{Rng, rngs::ThreadRng};
use std::{time::SystemTime, io};
pub struct Zixza {
    player: Player,
    board: Board,
    dices: Vec<Dice>,
    // dice1: Dice,
    // dice2: Dice,
    // dice3: Dice,
    // dice4: Dice,
    // dice5: Dice,
    // dice6: Dice,
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
            let mut top1 = 0;
            let mut top2 = 0;
            let mut rng = rand::thread_rng();
            let num1 = rng.gen_range(1..=6);
            let num2 = rng.gen_range(1..=6);
            if num1 == num2 {continue;}
            println!("Player1 : {}, Player2 : {}", num1, num2);
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
                if top1 > top2 { self.player = Player::P1;} else { self.player = Player::P2;}
                if self.player==Player::P1 {self.board.turnboard();}
                break;
            } else {
                top1 += num1;
                top2 += num2;
                continue;
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
            Dice::new(num as usize, top as usize, left as usize, getrightnum(top, left) as usize, player)
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
        'main: loop {
            self.board.step_count();
            println!("");
            println!("turn_count {}", self.board.getsteps());
            println!("dice [top,left right]");
            for v in &self.dices {
                v.show();
            }
            self.board.show();
            let (dice1, dice2) = self.dices.split_at(3);
            let player_dice = match self.player {
                Player::P1 => dice1.to_vec(),
                Player::P2 => dice2.to_vec(),
            };

            let player_dice_nums: Vec<usize> = player_dice.iter().map(|v| v.getnum()).collect();
            println!("Choose moving dice from {:?}", player_dice_nums);
            let (dice_num, dice_move) = 'select_dice: loop {
                let dice_num = input_usize();
                if player_dice_nums.iter().any(|v| *v==dice_num) {
                    loop {
                        let dice_moves = self.board.dice_move(dice_num, &self.dices);
                        println!("Choose a movement from");
                        for (i, v) in dice_moves.iter().enumerate() {
                            println!("{}: {}", i+1, v.to_string());
                        }
                        let input = input_usize();
                        if input > 0 && input <= dice_moves.len() {
                            break 'select_dice (dice_num, dice_moves[input-1]);
                        }
                        println!("again");
                    }
                }
            };

            match dice_move {
                Dice_move::ForwardLeft => {
                    self.dices[dice_num].forward_left();
                    self.board.forward_left(dice_num);
                },
                Dice_move::ForwardRight => {
                    self.dices[dice_num].forward_right();
                    self.board.forward_right(dice_num);
                },
                Dice_move::BackwardLeft => {
                    self.dices[dice_num].backward_left();
                    self.board.backward_left(dice_num);
                },
                Dice_move::BackwardRight => {
                    self.dices[dice_num].backward_right();
                    self.board.backward_right(dice_num);
                },
                Dice_move::TurnLeft => {
                    self.dices[dice_num].turn_left();
                },
                Dice_move::TurnRight => {
                    self.dices[dice_num].turn_right();
                },
            }
            self.player = if self.player==Player::P1 {Player::P2} else {Player::P1};
            self.board.turnboard();
        }
    }
    // 勝利条件
    
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



