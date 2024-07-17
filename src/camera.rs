//! Defines a camera to view the world

/// Flag if set draws chunk borders
pub const DebugChunks: u8 = 1 << 0;
/// Flag if set draws quad borders
pub const DebugQuad: u8 = 1 << 1;
/// Flag if set draw block selected
pub const DrawSelectionBox: u8 = 1 << 2;

type Flags = u8;

pub struct Camera {
    x: i64,
    y: i64,
    zoom: f32,
    mode: Flags,
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
}
/// Functions to draw and update camera
impl Camera {
    pub fn draw(&self) {}
}
