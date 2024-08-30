use std::f32::consts::PI;

use gust_math::vectors::vect3::Vect3;
use crate::components::viewer::Viewer;

pub struct Camera {
    pub id: u32,
    pub position: Vect3,
    pub direction: Vect3,
    pub up: Vect3,
    pub fov: f32,
    pub z_far: f32,
    pub z_near: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(
        id: u32,
        position: Vect3,
        direction: Vect3,
        up: Vect3,
        fov: f32,
        z_far: f32,
        z_near: f32,
        aspect_ratio: f32,
    ) -> Camera {
        Camera {
            id,
            position,
            direction,
            up,
            fov,
            z_far,
            z_near,
            aspect_ratio,
        }
    }

    pub fn init() -> Camera {
        Camera {
            id: 0,
            position: Vect3::new(0.0, 0.0, 0.0),
            direction: Vect3::new(0.0, 1.0, 0.0),
            up: Vect3::new(0.0, 0.0, 1.0),
            fov: PI / 3.0,
            z_far: 1024.0,
            z_near: 0.1,
            aspect_ratio: 480.0 / 800.0,
        }
    }
}

impl Viewer for Camera {
    fn position(&self) -> Vect3 {
        self.position.clone()
    }

    fn direction(&self) -> Vect3 {
        self.direction.clone()
    }

    fn up(&self) -> Vect3 {
        self.up.clone()
    }

    fn fov(&self) -> f32 {
        self.fov
    }

    fn z_far(&self) -> f32 {
        self.z_far
    }

    fn z_near(&self) -> f32 {
        self.z_near
    }

    fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}