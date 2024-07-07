use std::f32::consts::PI;

use gust_math::matrix::Matrix;
use gust_math::vect::Vect;

pub struct Camera {
    pub id: u32,
    pub position: Vect,
    pub direction: Vect,
    pub up: Vect,
    pub fov: f32,
    pub z_far: f32,
    pub z_near: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(
        id: u32,
        position: Vect,
        direction: Vect,
        up: Vect,
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
            position: Vect::from_slice(&[0.0, 0.0, 0.0]),
            direction: Vect::from_slice(&[0.0, 0.0, 0.0]),
            up: Vect::from_slice(&[0.0, 0.0, 0.0]),
            fov: PI / 3.0,
            z_far: 1024.0,
            z_near: 0.1,
            aspect_ratio: 480.0 / 800.0,
        }
    }

    pub fn get_perspective(&self) -> Matrix {
        let f = 1.0 / (self.fov / 2.0).tan();
        Matrix::from_slices(&[
            &[f * self.aspect_ratio, 0.0, 0.0, 0.0],
            &[0.0, f, 0.0, 0.0],
            &[0.0, 0.0, (self.z_far + self.z_near) / (self.z_far - self.z_near), 1.0],
            &[0.0, 0.0, -(2.0 * self.z_far * self.z_near) / (self.z_far - self.z_near), 0.0],
        ])
    }
}