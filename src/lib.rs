use zixza::Zixza;
use wasm_bindgen::prelude::*;

mod zixza;
mod collect_data;

#[wasm_bindgen]
pub fn game() {
    let mut game = Zixza::new();
    game.reset();
    
}
