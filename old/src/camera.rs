use macroquad::prelude::*;

pub struct Camera {
    pub camera: Camera2D,
    pub target: Vec2,
    pub zoom: f32,
    pub dragging: bool,
    pub last_mouse: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        let camera = Camera2D::default();
        set_camera(&camera);
        Self {
            camera,
            target: vec2(0.0, 0.0),
            zoom: 1.0,
            dragging: false,
            last_mouse: vec2(0.0, 0.0),
        }
    }

    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        self.camera.screen_to_world(screen)
    }

    pub fn apply(&mut self) {
        self.camera.zoom = Vec2 {
            x: self.zoom / screen_width(),
            y: self.zoom / screen_height(),
        };
        self.camera.target = self.target;
        set_camera(&self.camera);
    }
}
