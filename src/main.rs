use macroquad::prelude::{draw_text, get_fps, next_frame, BLACK};

use rand::{self, Rng};
use sand_engine::*;

#[macroquad::main("Sand Engine")]
async fn main() {
    let seed = rand::thread_rng().gen_range(0..u32::MAX);

    let mut camera = Camera::new(DEBUG_CHUNKS | DEBUG_MENU | DEBUG_QUAD | DRAW_SELECTION_BOX);
    let mut manager = WorldManager::new(seed);

    loop {
        camera.draw(&mut manager);
        camera.update();
        draw_text(format!("FPS: {}", get_fps()).as_str(), 16., 32., 32., BLACK);
        next_frame().await
    }
}
