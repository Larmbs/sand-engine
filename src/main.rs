use macroquad::{
    prelude::{
        clear_background, draw_rectangle, draw_rectangle_lines, draw_text, get_fps, is_key_down,
        next_frame, Color, KeyCode, BLACK, BLUE, RED,
    },
    window,
};
use sand_engine::*;
use rand::{self, Rng};

struct WorldWindow {
    seed: u32,
    zoom: f32,
    debug: bool,
    load_radius: usize,
    camera_x: i64,
    camera_y: i64,
    regions: Vec<(i32, i32, Region, Vec<Option<ChunkMesh>>)>,
}
impl WorldWindow {
    pub fn new(seed: u32, load_radius: usize) -> Self {
        WorldWindow {
            seed,
            zoom: 10.,
            debug: false,
            load_radius,
            camera_x: 0,
            camera_y: 0,
            regions: Vec::new(),
        }
    }
    pub fn load(&mut self) {
        let camera_region_pos_x = self.camera_x >> 8;
        let camera_region_pos_y = self.camera_y >> 8;
        let region = Region::new(
            camera_region_pos_x as i32,
            camera_region_pos_y as i32,
            self.seed,
        );
        let mut meshes = Vec::new();
        for chunk in region.chunks.iter() {
            meshes.push(match chunk {
                Some(chunk) => Some(ChunkMesh::from_chunk(&chunk.1)),
                None => None,
            });
        }
        self.regions.push((
            camera_region_pos_x as i32,
            camera_region_pos_y as i32,
            region,
            meshes,
        ))
    }
    pub fn draw(&self) {
        for (region_x, region_y, _, chunk_meshes) in &self.regions {
            for (i, mesh) in chunk_meshes.iter().enumerate() {
                match mesh {
                    Some(mesh) => {
                        let world_x =
                            self.camera_x - (region_x << 8 | ((i % 16) << 4) as i32) as i64;
                        let world_y =
                            self.camera_y - (region_y << 8 | ((i / 16) << 4) as i32) as i64;

                        draw_chunk_mesh(mesh, world_x as f32, world_y as f32, self.zoom, self.debug)
                    }
                    None => {}
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
        if is_key_down(KeyCode::Z) {
            self.zoom *= 0.9;
        }
        if is_key_down(KeyCode::X) {
            self.zoom *= 1.1;
        }
    }
}

#[macroquad::main("Sand Engine")]
async fn main() {
    let mut world_window = WorldWindow::new(rand::thread_rng().gen_range(0..u32::MAX), 32);
    world_window.load();

    loop {
        clear_background(Color::from_hex(0xa4e7f5));
        world_window.draw();
        world_window.update_camera();
        draw_text(format!("FPS: {}", get_fps()).as_str(), 16., 32., 32., BLACK);

        next_frame().await
    }
}

fn draw_chunk_mesh(chunk_mesh: &ChunkMesh, world_x: f32, world_y: f32, scale: f32, debug: bool) {
    for (block, rect) in chunk_mesh.mesh.iter() {
        draw_rectangle(
            (world_x + rect.x) * scale + window::screen_width() / 2.,
            (world_y + rect.y) * scale + window::screen_height() / 2.,
            rect.w * scale,
            rect.h * scale,
            block.get_color(),
        );
        if debug {
            draw_rectangle_lines(
                (world_x + rect.x) * scale + window::screen_width() / 2.,
                (world_y + rect.y) * scale + window::screen_height() / 2.,
                rect.w * scale,
                rect.h * scale,
                2.,
                RED,
            );
        }
    }
    if debug {
        draw_rectangle_lines(
            world_x * scale + window::screen_width() / 2.,
            world_y * scale + window::screen_height() / 2.,
            16. * scale,
            16. * scale,
            2.,
            BLUE,
        );
    }
}
