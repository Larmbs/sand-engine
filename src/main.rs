use macroquad::prelude::next_frame;

use rand::{self, Rng};
use sand_engine::*;

#[macroquad::main("Sand Engine")]
async fn main() {
    let seed = rand::thread_rng().gen_range(0..u32::MAX);
 
    let mut camera = Camera::new(flags::DEBUG_MENU | flags::DRAW_SELECTION_BOX | flags::CLAMP_ZOOM | flags::DEBUG_CHUNKS);
    let mut manager = WorldManager::new(seed);
    let mut controller = InspectController::new(0, 0);
    loop {
        camera.draw(&mut manager);
        controller.update(&mut camera, &mut manager);
        manager.clean();
        next_frame().await
    }
}
