use super::dice::{Player, Dice};
#[derive(PartialEq, Clone, Copy)]
pub enum DiceMove {
    ForwardLeft,
    ForwardRight,
    BackwardLeft,
    BackwardRight,
    TurnLeft,
    TurnRight,
    BeforeMove,
}
impl DiceMove {
    pub fn to_string(&self) -> &str{
        match self {
            DiceMove::ForwardLeft => "Forward_Left",
            DiceMove::ForwardRight => "Forward_Right",
            DiceMove::BackwardLeft => "Backward_Left",
            DiceMove::BackwardRight => "Backward_Right",
            DiceMove::TurnLeft => "Turn_Left",
            DiceMove::TurnRight => "Turn_Right",
            DiceMove::BeforeMove => "BeforeMove",
        }
    }
}
#[derive(Clone, Copy)]
pub enum BoardState {
    BeforeMatch,
    InMatch,
    Seizure,
    Occupation,
    Reach,
    Draw,
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
    movementbefore_p1: DiceMove,
    movementbefore_p2: DiceMove,
    sameboardcount: usize,
    boardstate: BoardState,
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
        ]), movementbefore_p1: (DiceMove::BeforeMove), movementbefore_p2: (DiceMove::BeforeMove), sameboardcount: (0), boardstate: (BoardState::BeforeMatch) }
    }
    pub fn show(&self) {
        println!("  A B C D E F G");
        for i in 0 ..self.board.len() {
            let row: String = self.board[i].iter().map(|v| v.to_char().to_string() + " ").collect();
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
    pub fn dice_move(&mut self, dice_num: usize, dices: &Vec<Dice>) -> (Vec<DiceMove>, usize) { //usize=>attack?
        let mut dicemoves: Vec<DiceMove> = Vec::new();
        let mut attack = 6;
        let (enemy_dices, beforemove) = if dice_num < 4 { (vec![Piece::Dice4, Piece::Dice5, Piece::Dice6], self.movementbefore_p1)} else {(vec![Piece::Dice1, Piece::Dice2, Piece::Dice3], self.movementbefore_p2)};
        let dice_position = self.get_dice_position(dice_num);
        if dice_position[1] != 0 && beforemove != DiceMove::BackwardRight {if self.board[dice_position[0]][dice_position[1]-1] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]][dice_position[1]-1]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dicemoves.push(DiceMove::ForwardLeft);
                    attack = enemy_dice_position;
                }
            }
            else {
                dicemoves.push(DiceMove::ForwardLeft);
            }
        }}
        if dice_position[0] != 0 && beforemove != DiceMove::BackwardLeft {if self.board[dice_position[0]-1][dice_position[1]] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]-1][dice_position[1]]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dicemoves.push(DiceMove::ForwardRight);
                    attack = enemy_dice_position;
                }
            }
            else {
                dicemoves.push(DiceMove::ForwardRight);
            }
        }}
        if dice_position[0] != 6 && beforemove != DiceMove::ForwardRight {if self.board[dice_position[0]+1][dice_position[1]] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]+1][dice_position[1]]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dicemoves.push(DiceMove::BackwardLeft);
                    attack = enemy_dice_position;
                }
            }
            else {
                dicemoves.push(DiceMove::BackwardLeft);
            }
        }}
        if dice_position[1] != 6 && beforemove != DiceMove::ForwardLeft {if self.board[dice_position[0]][dice_position[1]+1] != Piece::Wall {
            if let Some(mut enemy_dice_position) = enemy_dices.iter().position(|v| *v==self.board[dice_position[0]][dice_position[1]+1]) {
                if dice_num < 4 {enemy_dice_position+=3};
                if dices[dice_num-1].gettop() > dices[enemy_dice_position].gettop() {
                    dicemoves.push(DiceMove::BackwardRight);
                    attack = enemy_dice_position;
                }
            }
            else {
                dicemoves.push(DiceMove::BackwardRight);
            }
        }}
        if beforemove != DiceMove::TurnRight {dicemoves.push(DiceMove::TurnLeft);}
        if beforemove != DiceMove::TurnLeft {dicemoves.push(DiceMove::TurnRight);}
        (dicemoves, attack)
    }
    pub fn forward_left(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]][dice_position[1]-1] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
        self.sameboardcount=0;
    }
    pub fn forward_right(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]-1][dice_position[1]] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
        self.sameboardcount=0;
    }
    pub fn backward_left(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]+1][dice_position[1]] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
        self.sameboardcount=0;
    }
    pub fn backward_right(&mut self, dice_num: usize) {
        let dice_position = self.get_dice_position(dice_num);
        self.board[dice_position[0]][dice_position[1]+1] = self.board[dice_position[0]][dice_position[1]];
        self.board[dice_position[0]][dice_position[1]] = Piece::Empty;
        self.sameboardcount=0;
    }
    pub fn rewind(&mut self, player: Player, dicemove: DiceMove) {
        match player {
            Player::P1 => self.movementbefore_p1 = dicemove,
            Player::P2 => self.movementbefore_p2 = dicemove,
        }
    }
    pub fn getsameboardcount(&self) -> usize {
        self.sameboardcount
    }
    pub fn sameboard_count(&mut self) {
        self.sameboardcount += 1;
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
    pub fn win_check(&mut self, player: Player) {
        let center = vec![[2, 4], [3, 3], [4, 2]];
        let (p2, p1) = (vec![Piece::Dice4, Piece::Dice5, Piece::Dice6], vec![Piece::Dice1, Piece::Dice2, Piece::Dice3]);
        match player {
            Player::P1 => {
                if p1.iter().any(|w| *w==self.board[center[0][0]][center[0][1]]) && p1.iter().any(|w| *w==self.board[center[1][0]][center[1][1]]) && p1.iter().any(|w| *w==self.board[center[2][0]][center[2][1]]){
                    self.boardstate = BoardState::Occupation;
                }
                if p1.iter().any(|w| *w==self.board[1][1]) {
                    self.boardstate = BoardState::Reach;
                }
            }
            Player::P2 => {
                if p2.iter().any(|w| *w==self.board[center[0][0]][center[0][1]]) && p2.iter().any(|w| *w==self.board[center[1][0]][center[1][1]]) && p2.iter().any(|w| *w==self.board[center[2][0]][center[2][1]]){
                    self.boardstate = BoardState::Occupation;
                }
                if p2.iter().any(|w| *w==self.board[1][1]) {
                    self.boardstate = BoardState::Reach;
                }
            }
        }
    }
    pub fn setboardstate(&mut self, boardstate: BoardState) {
        self.boardstate = boardstate;
    }
    pub fn getboardstate(&mut self) -> BoardState{
        self.boardstate
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
    line_str + row_str
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
