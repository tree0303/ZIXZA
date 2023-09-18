use super::dice::Player;

#[derive(PartialEq)]
pub enum Piece {
    Dice1,
    Dice2,
    Dice3,
    Dice4,
    Dice5,
    Dice6,
    Empty,
    Wall,
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
        }
    }
}

pub struct Board {
    steps: u64, //ターン数
    board: Vec<Vec<Piece>>,
}
impl Board {
    pub fn new() -> Self {
        Self { steps: (0), board: (vec![
            vec![Piece::Wall ,Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Wall ,Piece::Wall ,Piece::Wall ],
            vec![Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Wall ,Piece::Wall ],
            vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Wall ],
            vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty],
            vec![Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty],
            vec![Piece::Wall ,Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Wall ],
            vec![Piece::Wall ,Piece::Wall ,Piece::Wall ,Piece::Empty,Piece::Empty,Piece::Wall ,Piece::Wall ],
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
}
pub fn getcoordinate(line: usize, row: usize) -> String{
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
