#[derive(PartialEq, Clone, Copy)]
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

pub struct Dice {
    num: u64,
    top: u64,
    left: u64,
    right: u64,
    player: Player,
}

impl Dice {
    pub fn new(num: u64, top: u64, left: u64, right: u64, player: Player) -> Self{
        Self { num: (num), top: (top), left: (left), right: (right), player: (player) }
    }
}

