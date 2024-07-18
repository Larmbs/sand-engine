//! Defines a camera to view the world

use crate::{space_conversion, WorldManager};

use super::{Block, ChunkMesh};
use macroquad::{
    prelude::{
        draw_line, draw_rectangle, draw_rectangle_lines, draw_text, get_fps,
        gl_use_default_material, gl_use_material, is_key_down, load_material, mouse_position,
        Color, KeyCode, Material, MaterialParams, ShaderSource, UniformType, BLACK, BLUE, PINK,
        RED, WHITE,
    },
    window,
};

/// Flag if set draws chunk borders
pub const DEBUG_CHUNKS: u8 = 1 << 0;
/// Flag if set draws quad borders
pub const DEBUG_QUAD: u8 = 1 << 1;
/// Flag if set draw block selected
pub const DRAW_SELECTION_BOX: u8 = 1 << 2;
/// Show Debug Menu (FPS Counter)
pub const DEBUG_MENU: u8 = 1 << 3;

/* Colors for debug lines */
const DEBUG_CHUNK_COLOR: Color = BLUE;
const DEBUG_QUAD_COLOR: Color = RED;
const SELECT_BOX_COLOR: Color = PINK;

type Flags = u8;

pub struct Camera {
    x: i64,
    y: i64,
    zoom: f32,
    flags: Flags,
    bg_mat: Material,
}
/// Functions for controlling camera
impl Camera {
    pub fn new(flags: Flags) -> Self {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("../shaders/gradient_vertex_shader.glsl"),
                fragment: include_str!("../shaders/gradient_fragment_shader.glsl"),
            },
            MaterialParams {
                uniforms: vec![("offset".to_string(), UniformType::Float1)],
                ..Default::default()
            },
        )
        .unwrap();
        Camera {
            zoom: 10.,
            x: 0,
            y: 0,
            bg_mat: material,
            flags,
        }
    }
    pub fn set_pos(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
    pub fn set_flags(&mut self, flags: Flags) {
        self.flags = flags;
    }
}
/// Functions to draw and update camera
impl Camera {
    pub fn update(&mut self) {
        let mut move_speed = 1;
        if is_key_down(KeyCode::LeftShift) {
            move_speed = 5;
        }
        if is_key_down(KeyCode::W) {
            self.y += move_speed;
        }
        if is_key_down(KeyCode::S) {
            self.y -= move_speed;
        }
        if is_key_down(KeyCode::D) {
            self.x -= move_speed;
        }
        if is_key_down(KeyCode::A) {
            self.x += move_speed;
        }
        if is_key_down(KeyCode::Z) {
            self.zoom *= 0.9;
        }
        if is_key_down(KeyCode::X) {
            self.zoom *= 1.1;
        }
    }
    pub fn draw(&self, manager: &mut WorldManager) {
        self.draw_background();

        let chunk_dim = self.zoom * 16.;
        for x in -1..(window::screen_width()/chunk_dim) as isize + 1 {
            for y in -1..(window::screen_height()/chunk_dim) as isize + 1 {
                let screen_x = x as f32 * chunk_dim;
                let screen_y = y as f32 * chunk_dim;
                let (world_x, world_y) = self.screen_to_world_cord(screen_x, screen_y);
                let (region_x, region_y) = space_conversion::get_region_cords(&world_x, &world_y);
                let (chunk_region_x, chunk_region_y) = space_conversion::get_region_chunk_cords(&world_x, &world_y);

                let (chunk_cord_x, chunk_cord_y) = space_conversion::get_chunk_world_cords(&world_x, &world_y);
                let rel_world_x = (self.x - chunk_cord_x) as f32;
                let rel_world_y = (self.y - chunk_cord_y) as f32;
                self.draw_chunk_mesh(
                    manager.get_chunk_mesh(&region_x, &region_y, &chunk_region_x, &chunk_region_y),
                    rel_world_x as f32,
                    rel_world_y as f32,
                );
            }
        }

        if self.flags & DRAW_SELECTION_BOX > 0 {
            self.draw_selected_block()
        }

        if self.flags & DEBUG_MENU > 0 {
            self.draw_debug_menu(manager.get_region_count())
        }
    }
}
impl Camera {
    pub fn screen_to_world_cord(&self, screen_x: f32, screen_y: f32) -> (i64, i64) {
        let center_x = window::screen_width() / 2.;
        let center_y = window::screen_height() / 2.;
        (
            self.x - ((screen_x - center_x) / self.zoom) as i64,
            self.y - ((screen_y - center_y) / self.zoom) as i64,
        )
    }
    /// Determines if a world coordinate is in view
    pub fn is_coordinate_visible(&self, world_x: i64, world_y: i64) -> bool {
        (self.x - world_x) as f32 * self.zoom <= window::screen_width() / 2.
            && (self.y - world_y) as f32 * self.zoom <= window::screen_height() / 2.
    }
    /// Draws a chunk mesh to the screen
    fn draw_chunk_mesh(&self, chunk_mesh: &ChunkMesh, rel_world_x: f32, rel_world_y: f32) {
        static DEBUG_LINE_WIDTH: f32 = 2.;
        let center_x = window::screen_width() / 2.;
        let center_y = window::screen_height() / 2.;

        for (block, rect) in chunk_mesh.mesh.iter() {
            let screen_x = (rel_world_x + rect.x) * self.zoom + center_x;
            let screen_y = (rel_world_y + rect.y) * self.zoom + center_y;
            draw_rectangle(
                screen_x,
                screen_y,
                rect.w * self.zoom,
                rect.h * self.zoom,
                block.color(),
            );
            if block == &Block::WaterEdge {
                draw_line(
                    screen_x,
                    screen_y,
                    screen_x + rect.w * self.zoom,
                    screen_y,
                    self.zoom / 4.,
                    WHITE,
                )
            }
            if self.flags & DEBUG_QUAD > 0 {
                draw_rectangle_lines(
                    screen_x,
                    screen_y,
                    rect.w * self.zoom,
                    rect.h * self.zoom,
                    DEBUG_LINE_WIDTH,
                    DEBUG_QUAD_COLOR,
                );
            }
        }
        if self.flags & DEBUG_CHUNKS > 0 {
            draw_rectangle_lines(
                (rel_world_x) * self.zoom + center_x,
                (rel_world_y) * self.zoom + center_y,
                16. * self.zoom,
                16. * self.zoom,
                DEBUG_LINE_WIDTH,
                DEBUG_CHUNK_COLOR,
            );
        }
    }
    /// Draws mouse selection box
    fn draw_selected_block(&self) {
        let mouse_pos = mouse_position();
        let world_x = ((mouse_pos.0 - window::screen_width() / 2.) / self.zoom).floor();
        let world_y = ((mouse_pos.1 - window::screen_height() / 2.) / self.zoom).floor();
        let screen_x = world_x * self.zoom + window::screen_width() / 2.;
        let screen_y = world_y * self.zoom + window::screen_height() / 2.;

        draw_rectangle_lines(
            screen_x,
            screen_y,
            self.zoom,
            self.zoom,
            6.,
            SELECT_BOX_COLOR,
        )
    }
    /// Draws a gradient background
    fn draw_background(&self) {
        self.bg_mat
            .set_uniform("offset", (self.y as f32 / self.zoom) / 256.);
        gl_use_material(&self.bg_mat);
        draw_rectangle(
            0.,
            0.,
            window::screen_width(),
            window::screen_height(),
            WHITE,
        );
        gl_use_default_material();
    }
    /// Draws debug menu
    fn draw_debug_menu(&self, regions: usize) {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 16., 32., 32., BLACK);
        draw_text(format!("Regions Loaded: {}", regions).as_str(), 16., 50., 32., BLACK);
    }
}
