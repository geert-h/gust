use gust_math::vectors::vect3::Vect3;

pub struct CameraComponent {
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub aspect_ratio: f32,
}

impl CameraComponent {
    pub fn new(fov: f32, z_near: f32, z_far: f32, aspect_ratio: f32) -> Self {
        CameraComponent {
            fov,
            z_near,
            z_far,
            aspect_ratio,
        }
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let f = 1.0 / (self.fov / 2.0).tan();
        [
            [f * self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (self.z_far + self.z_near) / (self.z_far - self.z_near), 1.0],
            [0.0, 0.0, -(2.0 * self.z_far * self.z_near) / (self.z_far - self.z_near), 0.0],
        ]
    }

    pub fn view_matrix(&self, position: Vect3, forward: Vect3, up: Vect3) -> [[f32; 4]; 4] {
        let f = forward;
        let s = up.cross(&f).normalize();
        let u = f.cross(&s).normalize();

        let p = [
            -position.dot(&s),
            -position.dot(&u),
            -position.dot(&f),
        ];

        [
            [s.x, u.x, f.x, 0.0],
            [s.y, u.y, f.y, 0.0],
            [s.z, u.z, f.z, 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}