pub mod board;
use crate::zixza::{board::Board, dice::Dice};
pub mod dice;
use crate::zixza::dice::Player;
use rand::{Rng, rngs::ThreadRng};
use std::{time::SystemTime, io};
pub struct Zixza {
    player: Player,
    board: Board,
    // dice1: Dice,
    // dice2: Dice,
    // dice3: Dice,
    // dice4: Dice,
    // dice5: Dice,
    // dice6: Dice,
}

impl Zixza {
    pub fn new() -> Self {
        let mut board = Board::new();
        let mut dice1;
        let mut dice2;
        let mut dice3;
        let mut dice4;
        let mut dice5;
        let mut dice6;
        let mut count = 0;
        board.show();
        loop {
            let mut rng = rand::thread_rng();
            let num1 = rng.gen_range(1..=6);
            let num2 = rng.gen_range(1..=6);
            if num1 == num2 {continue;}
            println!("Player1 : {}, Player2 : {}", num1, num2);
            let mut player = if num1 > num2 {Player::P1} else {Player::P2};
            count += 1;
            match count {
                1 => dice1 = gendice(player, count, num1),
                2 => dice2 = gendice(player, count, num1),
                3 => dice3 = gendice(player, count, num1),
                _ => continue
            }
            player = if player==Player::P1 {Player::P2} else {Player::P1};
            match count {
                1 => dice4 = gendice(player, &count+3, num1),
                2 => dice5 = gendice(player, &count+3, num1),
                3 => dice6 = gendice(player, &count+3, num1),
                _ => continue
            }
            if count == 3 {break;}
        }
        Self { player: (Player::P1), board: (board) }    
    }
        
}

// pub setdice(board: Board, player: Player) {
//     let emp_places = Board::initplace(board, player);

// }

pub fn gendice(player: Player, num: usize, top: usize) -> Dice {
    println!("{}", player.to_string());
    println!("Top_Number is {}",top);
    let mut left = loop {
        println!("Choose a left_number from{:?}",getsidenums(top));
        let input = input_usize();
        if getsidenums(top).iter().any(|v| v==&input) {
            break input;
        }
    };
    Dice::new(num as u64, top as u64, left as u64, getrightnum(top, left) as u64, player)
}
pub fn input_usize() -> usize {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<usize>() {
                    Ok(n) => break n,
                    Err(_) => {
                        println!("again");
                        continue;
                    }
                }
            }
            Err(_) => {
                continue;
            }
        };
        
    }
    
}
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
pub fn getsidenums(topnum: usize) -> Vec<usize> {
    let mut sidenums: Vec<usize> = Vec::new();
    for n in 1..=6{
        if n == topnum || n == getbacknum(topnum){continue;}
        sidenums.push(n);
    }
    return sidenums;
} 
pub fn getbacknum(num: usize) -> usize {
    return 7 - num;
}
