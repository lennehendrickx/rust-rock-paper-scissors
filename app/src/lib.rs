mod utils;
mod game;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use crate::game::Game;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate lazy_static;

//Global variable
lazy_static! {
    static ref GAME: Mutex<Game> = {
        Mutex::new(Game::new())
    };
}

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

//Imported JS function from index.html
#[wasm_bindgen]
extern "C" {
    fn draw_particle(x: f64, y: f64, s: &str, size: f64);
}

//Exported Rust functions used by index.html
#[wasm_bindgen]
pub fn game_init(width: f64, height: f64) {
    GAME.lock().unwrap().init(width, height);
}

#[wasm_bindgen]
pub fn game_update(timestamp: f64) {
    GAME.lock().unwrap().update(timestamp);
    GAME.lock().unwrap().detect_collisions();
    GAME.lock().unwrap().render(|square| draw_particle(square.x, square.y, &square.color, square.size));
}
