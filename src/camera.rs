//! Defines a camera to view the world

use super::ChunkMesh;
use crate::{conversion, WorldManager};
use macroquad::{
    prelude::{
        draw_rectangle, draw_rectangle_lines, draw_text, get_fps, gl_use_default_material,
        gl_use_material, load_material, mouse_position, Color, Material, MaterialParams,
        ShaderSource, UniformType, BLACK, BLUE, PINK, RED, WHITE,
    },
    window,
};

pub mod flags {
    pub const DEBUG_CHUNKS: u8 = 1 << 0;
    pub const DEBUG_QUADS: u8 = 1 << 1;
    pub const DRAW_SELECTION_BOX: u8 = 1 << 2;
    pub const DEBUG_MENU: u8 = 1 << 3;
    pub const CLAMP_ZOOM: u8 = 1 << 4;
}

const DEBUG_CHUNK_COLOR: Color = BLUE;
const DEBUG_QUAD_COLOR: Color = RED;
const SELECT_BOX_COLOR: Color = PINK;
const DEBUG_LINE_WIDTH: f32 = 2.0;

type Flags = u8;

pub struct Camera {
    x: i64,
    y: i64,
    zoom: f32,
    flags: Flags,
    bg_mat: Material,
    chunks_drawn: usize,
}

impl Camera {
    pub fn new(flags: Flags) -> Self {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: include_str!("../assets/shaders/gradient_bg_vertex_shader.glsl"),
                fragment: include_str!("../assets/shaders/gradient_bg_fragment_shader.glsl"),
            },
            MaterialParams {
                uniforms: vec![("offset".to_string(), UniformType::Float1)],
                ..Default::default()
            },
        )
        .expect("Error loading shaders");
        Camera {
            zoom: 30.0,
            x: 0,
            y: 0,
            bg_mat: material,
            flags,
            chunks_drawn: 0,
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

    pub fn draw(&mut self, manager: &mut WorldManager) {
        self.draw_background();

        let chunk_dim = self.zoom * 16.0;
        self.chunks_drawn = 0;

        let screen_width_chunks = (window::screen_width() / chunk_dim) as isize;
        let screen_height_chunks = (window::screen_height() / chunk_dim) as isize;

        for x in -1..=screen_width_chunks {
            for y in -1..=screen_height_chunks {
                let screen_x = x as f32 * chunk_dim;
                let screen_y = y as f32 * chunk_dim;
                let (world_x, world_y) = self.screen_to_world_cord(screen_x, screen_y);
                let (region_x, region_y) = conversion::get_region_cords(&world_x, &world_y);
                let (chunk_region_x, chunk_region_y) =
                    conversion::get_region_chunk_cords(&world_x, &world_y);
                let (chunk_cord_x, chunk_cord_y) =
                    conversion::get_chunk_world_cords(&world_x, &world_y);

                let rel_world_x = (self.x - chunk_cord_x) as f32;
                let rel_world_y = (self.y - chunk_cord_y) as f32;

                self.draw_chunk_mesh(
                    manager.get_chunk_mesh(&region_x, &region_y, &chunk_region_x, &chunk_region_y),
                    rel_world_x,
                    rel_world_y,
                );
                self.chunks_drawn += 1;
            }
        }

        if self.flags & flags::DRAW_SELECTION_BOX > 0 {
            self.draw_selected_block();
        }

        if self.flags & flags::DEBUG_MENU > 0 {
            self.draw_debug_menu(manager);
        }
    }

    fn screen_to_world_cord(&self, screen_x: f32, screen_y: f32) -> (i64, i64) {
        let center_x = window::screen_width() / 2.0;
        let center_y = window::screen_height() / 2.0;
        (
            self.x - ((screen_x - center_x) / self.zoom) as i64,
            self.y - ((screen_y - center_y) / self.zoom) as i64,
        )
    }

    fn draw_chunk_mesh(&self, chunk_mesh: &ChunkMesh, rel_world_x: f32, rel_world_y: f32) {
        let center_x = window::screen_width() / 2.0;
        let center_y = window::screen_height() / 2.0;

        for (color, rect) in chunk_mesh.mesh.iter() {
            // Flip x and y coordinates within the chunk
            let flipped_x = 16.0 - rect.x - rect.w;
            let flipped_y = 16.0 - rect.y - rect.h;

            let screen_x = (rel_world_x + flipped_x) * self.zoom + center_x;
            let screen_y = (rel_world_y + flipped_y) * self.zoom + center_y;

            draw_rectangle(
                screen_x,
                screen_y,
                rect.w * self.zoom,
                rect.h * self.zoom,
                *color,
            );

            if self.flags & flags::DEBUG_QUADS > 0 {
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

        if self.flags & flags::DEBUG_CHUNKS > 0 {
            draw_rectangle_lines(
                rel_world_x * self.zoom + center_x,
                rel_world_y * self.zoom + center_y,
                16.0 * self.zoom,
                16.0 * self.zoom,
                DEBUG_LINE_WIDTH,
                DEBUG_CHUNK_COLOR,
            );
        }
    }

    fn draw_selected_block(&self) {
        let mouse_pos = mouse_position();
        let world_x = ((mouse_pos.0 - window::screen_width() / 2.0) / self.zoom).floor();
        let world_y = ((mouse_pos.1 - window::screen_height() / 2.0) / self.zoom).floor();
        let screen_x = world_x * self.zoom + window::screen_width() / 2.0;
        let screen_y = world_y * self.zoom + window::screen_height() / 2.0;

        draw_rectangle_lines(
            screen_x,
            screen_y,
            self.zoom,
            self.zoom,
            6.0,
            SELECT_BOX_COLOR,
        );
    }

    fn draw_background(&self) {
        self.bg_mat
            .set_uniform("offset", (self.y as f32 / self.zoom) / 256.0);
        gl_use_material(&self.bg_mat);
        draw_rectangle(
            0.0,
            0.0,
            window::screen_width(),
            window::screen_height(),
            WHITE,
        );
        gl_use_default_material();
    }

    fn draw_debug_menu(&self, manager: &mut WorldManager) {
        let (mouse_x, mouse_y) = mouse_position();
        let (cursor_x, cursor_y) = self.screen_to_world_cord(mouse_x, mouse_y);
        let regions = manager.get_region_count();
        let block = manager.get_block(&-cursor_x, &cursor_y);

        let text = format!(
            "FPS: {}\nRegions Loaded: {}\nChunks Drawn: {}\nZoom Level: {}\nCamera X: {}\nCamera Y: {}\nCursor X: {}\nCursor Y: {}\nBlock: {:?}",
            get_fps(),
            regions,
            self.chunks_drawn,
            self.zoom,
            self.x,
            self.y,
            cursor_x,
            cursor_y,
            block
        );

        const PADDING: f32 = 5.0;
        const FONT_SIZE: f32 = 30.0;
        const LINE_SPACING: f32 = 5.0;
        for (i, line) in text.lines().enumerate() {
            draw_text(
                line,
                PADDING,
                PADDING + (i as f32 + 1.0) * (FONT_SIZE + LINE_SPACING) / 2.0,
                FONT_SIZE,
                BLACK,
            );
        }
    }
}
