//! Defines a camera to view the world

use std::collections::HashMap;

use super::{Block, ChunkMesh};
use macroquad::{
    prelude::{
        draw_line, draw_rectangle, draw_rectangle_lines, draw_text, get_fps,
        gl_use_default_material, gl_use_material, is_key_down, mouse_position, Color, KeyCode,
        Material, BLACK, BLUE, PINK, RED, WHITE,
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
    pub fn draw_chunks(&self, loaded_meshes: &HashMap<(i64, i64), ChunkMesh>) {
        self.draw_background();

        for (world_x, world_y) in loaded_meshes.keys() {
            if self.is_coordinate_visible(*world_x, *world_y) {
                self.draw_chunk_mesh(
                    loaded_meshes.get(&(*world_x, *world_y)).unwrap(),
                    *world_x as f32,
                    *world_y as f32,
                )
            }
        }

        if self.flags & DRAW_SELECTION_BOX > 0 {
            self.draw_selected_block()
        }

        if self.flags & DEBUG_MENU > 0 {
            self.draw_debug_menu()
        }
    }
}
impl Camera {
    /// Determines if a world coordinate is in view
    pub fn is_coordinate_visible(&self, world_x: i64, world_y: i64) -> bool {
        (self.x - world_x) as f32 * self.zoom <= window::screen_width() / 2.
            && (self.y - world_y) as f32 * self.zoom <= window::screen_height() / 2.
    }
    /// Draws a chunk mesh to the screen
    fn draw_chunk_mesh(&self, chunk_mesh: &ChunkMesh, world_x: f32, world_y: f32) {
        static DEBUG_LINE_WIDTH: f32 = 2.;
        let center_x = window::screen_width() / 2.;
        let center_y = window::screen_height() / 2.;

        for (block, rect) in chunk_mesh.mesh.iter() {
            let screen_x = (world_x + rect.x) * self.zoom + center_x;
            let screen_y = (world_y + rect.y) * self.zoom + center_y;
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
                (world_x) * self.zoom + center_x,
                (world_y) * self.zoom + center_y,
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
    fn draw_debug_menu(&self) {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 16., 32., 32., BLACK);
    }
}
