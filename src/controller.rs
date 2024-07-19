use super::Camera;
use crate::WorldManager;
use macroquad::prelude::{is_key_down, KeyCode};

/* Camera Options */
const MAX_ZOOM: f32 = 30.;
const MIN_ZOOM: f32 = 1.5;
const DEFAULT_ZOOM: f32 = 10.;

pub trait Controller {
    fn update(&mut self, camera: &mut Camera, manager: &mut WorldManager);
}

pub struct InspectController {
    x: i64,
    y: i64,
    zoom: f32,
}
impl InspectController {
    pub fn new(x: i64, y: i64) -> InspectController {
        InspectController {
            x,
            y,
            zoom: DEFAULT_ZOOM,
        }
    }
}
impl Controller for InspectController {
    fn update(&mut self, camera: &mut Camera, _manager: &mut WorldManager) {
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
        self.zoom = self.zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        camera.set_pos(self.x, self.y);
        camera.set_zoom(self.zoom);
    }
}

struct FixedPoint {
    pub large: i64,
    pub small: f32,
}
impl FixedPoint {
    fn move_by(&mut self, delta: f32) {
        let total_delta = (self.small + delta) as f64;
        let large_delta = total_delta.floor() as i64;
        let new_small = (total_delta - large_delta as f64) as f32;

        self.large += large_delta;
        self.small = new_small;
    }
}
pub struct PlayerController {
    x: FixedPoint,
    y: FixedPoint,
    vy: f32,
    zoom: f32,
}
impl PlayerController {
    pub fn new(x: i64, y: i64) -> PlayerController {
        PlayerController {
            x: FixedPoint {
                large: x,
                small: 0.,
            },
            y: FixedPoint {
                large: y,
                small: 0.,
            },
            vy: 0.,
            zoom: DEFAULT_ZOOM,
        }
    }
}
impl Controller for PlayerController {
    fn update(&mut self, camera: &mut Camera, manager: &mut WorldManager) {
        if manager
            .get_block(&self.x.large, &(self.y.large - 1))
            .is_solid()
        {
            self.vy = 0.;
            if is_key_down(KeyCode::Space) {
                self.vy = 3.;
            }
        } else {
            self.vy -= 0.1;
        }

        if is_key_down(KeyCode::A) {
            if !manager
                .get_block(&(self.x.large + 1), &self.y.large)
                .is_solid()
            {
                self.x.move_by(0.5);
            }
        }
        if is_key_down(KeyCode::D) {
            if !manager
                .get_block(&(self.x.large - 1), &self.y.large)
                .is_solid()
            {
                self.x.move_by(-0.5);
            }
        }
        if is_key_down(KeyCode::Z) {
            self.zoom *= 0.9;
        }
        if is_key_down(KeyCode::X) {
            self.zoom *= 1.1;
        }
        self.y.move_by(self.vy);
        self.zoom = self.zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        camera.set_pos(self.x.large, self.y.large);
        camera.set_zoom(self.zoom);
    }
}
