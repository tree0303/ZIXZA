use std::mem::swap;

use super::dice::{Player, Dice};
#[derive(PartialEq, Clone, Copy)]
pub enum Dice_move {
    ForwardLeft,
    ForwardRight,
    BackwardLeft,
    BackwardRight,
    TurnLeft,
    TurnRight,
}
impl Dice_move {
    pub fn to_string(&self) -> &str{
        match self {
            Dice_move::ForwardLeft => "Forward_Left",
            Dice_move::ForwardRight => "Forward_Right",
            Dice_move::BackwardLeft => "Backward_Left",
            Dice_move::BackwardRight => "Backward_Right",
            Dice_move::TurnLeft => "Turn_Left",
            Dice_move::TurnRight => "Turn_Right",
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Dice1,
    Dice2,
    Dice3,
    Dice4,
    Dice5,
    Dice6,
    Empty,
    Wall,
    Player1,
    Player2,
}
impl Piece {
    fn to_char(&self) -> char {
        match self {
            Piece::Dice1 => '1',
            Piece::Dice2 => '2',
            Piece::Dice3 => '3',
            Piece::Dice4 => '4',
            Piece::Dice5 => '5',
            Piece::Dice6 => '6',
            Piece::Empty => '0',
            Piece::Wall  => '*',
            Piece::Player1 => 'a',
            Piece::Player2 => 'b',
        }
    }
    
}

pub struct Board {
    steps: usize, //ターン数
    board: Vec<Vec<Piece>>,
    // 一手番前の動きp1
    // 一手番前の動きp2
}
impl Board {
    pub fn new() -> Self {
        Self { steps: (0), board: (vec![
            vec![Piece::Player1 ,Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Wall ,Piece::Wall ,Piece::Wall ],
            vec![Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Wall ,Piece::Wall ],
            vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Wall ],
            vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty],
            vec![Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty],
            vec![Piece::Wall ,Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Wall ],
            vec![Piece::Wall ,Piece::Wall ,Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Wall ,Piece::Player2 ],
        ]) }
    }
    pub fn show(&self) {
        println!("  A B C D E F G");
        for i in 0 ..self.board.len() {
            let mut row = String::new();
            row = self.board[i].iter().map(|v| v.to_char().to_string() + " ").collect();
            println!("{} {}",i, row);
        }
    }
    pub fn initplace(&self, player: Player) -> Vec<[usize; 2]> {
        let place = [[[0, 2], [1, 1], [2, 0]], [[6, 4], [5, 5], [4, 6]]];
        let mut emp_places = Vec::new();
        match player {
            Player::P1 => {
                for [i, n] in place[0] {
                    if self.board[i][n] == Piece::Empty {
                        emp_places.push([i, n]);
                    }
                }
            },
            Player::P2 => {
                for [i, n] in place[1] {
                    if self.board[i][n] == Piece::Empty {
                        emp_places.push([i, n]);
                    }
                }
            }
        }
        return emp_places;
    }
    pub fn putdice(&mut self, place: [usize; 2], player: Player, count: usize) {
        match (player, count) {
            (Player::P1, 1) => self.board[place[0]][place[1]] = Piece::Dice1,
            (Player::P1, 2) => self.board[place[0]][place[1]] = Piece::Dice2,
            (Player::P1, 3) => self.board[place[0]][place[1]] = Piece::Dice3,
            (Player::P2, 1) => self.board[place[0]][place[1]] = Piece::Dice4,
            (Player::P2, 2) => self.board[place[0]][place[1]] = Piece::Dice5,
            (Player::P2, 3) => self.board[place[0]][place[1]] = Piece::Dice6,
            (Player::P1, _) => self.board[place[0]][place[1]] = Piece::Empty,
            (Player::P2, _) => self.board[place[0]][place[1]] = Piece::Empty,
        }
    }
    pub fn turnboard(&mut self) {
        let size = self.board.len();
        for i in 0..size/2{
            for j in 0..size {
                let buf1 = self.board[i][j];
                let buf2 = self.board[size-i-1][size-j-1];
                self.board[i][j] = buf2;
                self.board[size-i-1][size-j-1] = buf1;
            }
        }
        for i in 0..size {
            let buf1 = self.board[3][i];
            let buf2 = self.board[3][size-i-1];
            self.board[3][i] = buf2;
            self.board[3][size-i-1] = buf1;
        }
    }
    pub fn dice_move(&mut self, dice_num: usize, dices: &Vec<Dice>) -> Vec<Dice_move> {
        let mut dice_moves: Vec<Dice_move> = Vec::new();
        let enemy_dices = if dice_num < 4 { vec![Piece::Dice4, Piece::Dice5, Piece::Dice6]} else {vec![Piece::Dice1, Piece::Dice2, Piece::Dice3]};
        let dice_position = self.get_dice_position(dice_num);
        if dice_position[1] != 0 {if self.board[dice_position[0]][dice_position[1]-1] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]][dice_position[1]-1]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dice_moves.push(Dice_move::ForwardLeft);
                }
            }
            else {
                dice_moves.push(Dice_move::ForwardLeft);
            }
        }}
        if dice_position[0] != 0 {if self.board[dice_position[0]-1][dice_position[1]] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]-1][dice_position[1]]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dice_moves.push(Dice_move::ForwardRight);
                }
            }
            else {
                dice_moves.push(Dice_move::ForwardRight);
            }
        }}
        if dice_position[0] != 6 {if self.board[dice_position[0]+1][dice_position[1]] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]+1][dice_position[1]]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dice_moves.push(Dice_move::BackwardLeft);
                }
            }
            else {
                dice_moves.push(Dice_move::BackwardLeft);
            }
        }}
        if dice_position[1] != 6 {if self.board[dice_position[0]][dice_position[1]+1] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]][dice_position[1]+1]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dice_moves.push(Dice_move::BackwardRight);
                }
            }
            else {
                dice_moves.push(Dice_move::BackwardRight);
            }
        }}
        
        dice_moves.push(Dice_move::TurnLeft);
        dice_moves.push(Dice_move::TurnRight);
        dice_moves
    }
    pub fn forward_left(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]][dice_position[1]-1] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
    }
    pub fn forward_right(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]-1][dice_position[1]] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
    }
    pub fn backward_left(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]+1][dice_position[1]] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
    }
    pub fn backward_right(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]][dice_position[1]+1] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
    }


    pub fn step_count(&mut self) {
        self.steps += 1;
    }
    pub fn getsteps(&self) -> usize{
        self.steps
    }
    pub fn get_dice_position(&mut self, dice_num: usize) -> [usize; 2] {
        let mut dice_position = [0, 0];
        'out: for (i,v) in self.board.iter().enumerate() {
            for (j, w) in v.iter().enumerate() {
                if *w == to_piece(dice_num){
                    dice_position = [i, j];
                    break 'out;
                }
            }
        }
        dice_position
    }

}
pub fn getcoordinate(line: &usize, row: &usize) -> String{
    let line_str = line.to_string();
    let row_str = match row {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "G",
        _ => "*"
    };
    (line_str + row_str)
}
pub fn to_piece(num: usize) -> Piece {
    match num {
        1 => Piece::Dice1,
        2 => Piece::Dice2,
        3 => Piece::Dice3,
        4 => Piece::Dice4,
        5 => Piece::Dice5,
        6 => Piece::Dice6,
        7 => Piece::Empty,
        8 => Piece::Player1,
        9 => Piece::Player2,
        _ => Piece::Wall,
    }
}
