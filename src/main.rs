use macroquad::{
    color::PINK,
    material::gl_use_default_material,
    prelude::{
        draw_rectangle, draw_rectangle_lines, draw_text, get_fps, gl_use_material, is_key_down,
        load_material, mouse_position, next_frame, KeyCode, MaterialParams, ShaderSource, BLACK,
        BLUE, RED, WHITE,
    },
    shapes::draw_line,
    window,
};

use rand::{self, Rng};
use sand_engine::*;

struct WorldWindow {
    seed: u32,
    zoom: f32,
    debug: bool,
    camera_x: i64,
    camera_y: i64,
    regions: Vec<(i32, i32, Region, Vec<Option<ChunkMesh>>)>,
}
impl WorldWindow {
    pub fn new(seed: u32) -> Self {
        WorldWindow {
            seed,
            zoom: 10.,
            debug: true,
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
                Some(chunk) => Some(ChunkMesh::greedy_mesh(&chunk.1)),
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

    /// Determines if world coordinate is visible
    pub fn is_coordinate_visible(&self, world_x: i64, world_y: i64) -> bool {
        (self.camera_x - world_x) as f32 * self.zoom <= window::screen_width() / 2.
            && (self.camera_y - world_y) as f32 * self.zoom <= window::screen_height() / 2.
    }

    pub fn draw(&self) {
        for (region_x, region_y, _, chunk_meshes) in &self.regions {
            for (i, mesh) in chunk_meshes.iter().enumerate() {
                match mesh {
                    Some(mesh) => {
                        let chunk_world_x = (region_x << 8) as i64 | ((i % 16) << 4) as i64;
                        let chunk_world_y = (region_y << 8) as i64 | ((i / 16) << 4) as i64;

                        let rel_world_x = (self.camera_x - chunk_world_x) as f32;
                        let rel_world_y = (self.camera_y - chunk_world_y) as f32;

                        if self.is_coordinate_visible(chunk_world_x, chunk_world_y) {
                            draw_chunk_mesh(mesh, rel_world_x, rel_world_y, self.zoom, self.debug)
                        }
                    }
                    None => {}
                }
            }
        }

        draw_selected_block(self.zoom);
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

fn draw_chunk_mesh(chunk_mesh: &ChunkMesh, world_x: f32, world_y: f32, scale: f32, debug: bool) {
    static DEBUG_LINE_WIDTH: f32 = 2.;

    for (block, rect) in chunk_mesh.mesh.iter() {
        let screen_x = (world_x + rect.x) * scale + window::screen_width() / 2.;
        let screen_y = (world_y + rect.y) * scale + window::screen_height() / 2.;
        draw_rectangle(
            screen_x,
            screen_y,
            rect.w * scale,
            rect.h * scale,
            block.color(),
        );
        if block == &Block::WaterEdge {
            draw_line(
                screen_x,
                screen_y,
                screen_x + rect.w * scale,
                screen_y,
                scale / 4.,
                WHITE,
            )
        }
        if debug {
            draw_rectangle_lines(
                screen_x,
                screen_y,
                rect.w * scale,
                rect.h * scale,
                DEBUG_LINE_WIDTH,
                RED,
            );
        }
    }
    if debug {
        draw_rectangle_lines(
            (world_x) * scale + window::screen_width() / 2.,
            (world_y) * scale + window::screen_height() / 2.,
            16. * scale,
            16. * scale,
            DEBUG_LINE_WIDTH,
            BLUE,
        );
    }
}

fn draw_selected_block(scale: f32) {
    let mouse_pos = mouse_position();
    let world_x = ((mouse_pos.0 - window::screen_width() / 2.) / scale).floor();
    let world_y = ((mouse_pos.1 - window::screen_height() / 2.) / scale).floor();
    let screen_x = world_x * scale + window::screen_width() / 2.;
    let screen_y = world_y * scale + window::screen_height() / 2.;

    draw_rectangle_lines(screen_x, screen_y, scale, scale, 6., PINK)
}

#[macroquad::main("Sand Engine")]
async fn main() {
    let seed = rand::thread_rng().gen_range(0..u32::MAX);
    let mut world_window = WorldWindow::new(seed);
    world_window.load();

    let gradient_material = load_material(
        ShaderSource::Glsl {
            vertex: include_str!("../shaders/gradient_vertex_shader.glsl"),
            fragment: include_str!("../shaders/gradient_fragment_shader.glsl"),
        },
        MaterialParams::default(),
    )
    .unwrap();

    loop {
        gl_use_material(&gradient_material);
        draw_rectangle(
            0.,
            0.,
            window::screen_width(),
            window::screen_height(),
            WHITE,
        );
        gl_use_default_material();
        world_window.draw();
        world_window.update_camera();
        draw_text(format!("FPS: {}", get_fps()).as_str(), 16., 32., 32., BLACK);

        next_frame().await
    }
}
