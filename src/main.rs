use macroquad::{prelude::*, window};
use sand_engine::*;


struct WorldWindow {
    seed: u32,
    zoom: f32,
    load_radius: usize,
    camera_x: i64,
    camera_y: i64,
    regions: Vec<(i32, i32, Region)>,
}
impl WorldWindow {
    pub fn new(seed: u32, load_radius: usize) -> Self {
        WorldWindow {
            seed,
            zoom: 5.,
            load_radius,
            camera_x: 0,
            camera_y: 0,
            regions: Vec::new(),
        }
    }
    pub fn load(&mut self) {
        let camera_region_pos_x = self.camera_x >> 8;
        let camera_region_pos_y = self.camera_y >> 8;

        self.regions.push((camera_region_pos_x as i32, camera_region_pos_y as i32, Region::new(camera_region_pos_x as i32, camera_region_pos_y as i32, self.seed)))
    }
    pub fn draw(&self) {
        let camera_region_pos_x = self.camera_x >> 4;
        let camera_region_pos_y = self.camera_y >> 4;
        for (region_x, region_y, region) in &self.regions {
            for (i, chunk) in region.chunks.iter().enumerate() {
                let chunk_x = i % 16;
                let chunk_y = i / 16;
                let chunk_region_x = region_x << 4 | chunk_x as i32;
                let chunk_region_y = region_y << 4 | chunk_y as i32;

                if (camera_region_pos_x - chunk_region_x as i64).pow(2) + 
                    (camera_region_pos_y - chunk_region_y as i64).pow(2) < 
                    self.load_radius as i64 {
                    match chunk {
                     Some((_, chunk)) => {
                        draw_chunk(chunk, 
                            (self.camera_x - (chunk_region_x << 4) as i64) as f32 * self.zoom, 
                            (self.camera_y - (chunk_region_y << 4) as i64) as f32 * self.zoom, 
                            self.zoom as f32)
                     },
                     None => {},
                    }
                }
            }
        }
    }
    pub fn update_camera(&mut self) {
        let mut move_speed = 1;
        if is_key_down(KeyCode::LeftShift) {
            move_speed = 5;
        }
        if is_key_down(KeyCode::W) {
            self.camera_y += move_speed;
        } 
        if is_key_down(KeyCode::S) {
            self.camera_y -= move_speed;
        }
        if is_key_down(KeyCode::D) {
            self.camera_x -= move_speed;
        } 
        if is_key_down(KeyCode::A) {
            self.camera_x += move_speed;
        }
    }
}




#[macroquad::main("Sand Engine")]
async fn main() {
    let mut world_window = WorldWindow::new(1, 20);
    world_window.load();

    loop {
        clear_background(Color::from_hex(0xa4e7f5));

        world_window.draw();
        world_window.update_camera();
        
        next_frame().await
    }
}

fn draw_chunk(chunk: &Chunk, screen_x: f32, screen_y: f32, scale: f32) {
    for (i, block) in chunk.blocks.iter().enumerate() {
        draw_rectangle(
            screen_x + (i % 16) as f32 * scale + window::screen_width() / 2.,
            screen_y + (i / 16) as f32 * scale + window::screen_height() / 2.,
            scale,
            scale,
            block.get_color(),
        )
    }
}