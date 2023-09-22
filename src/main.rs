mod zixza;
use crate::zixza::Zixza;

fn main() {
    println!("Hello, world!");
    let mut game = Zixza::new();
    game.setup();
    game.start();
}
