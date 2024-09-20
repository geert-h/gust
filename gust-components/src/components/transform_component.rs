use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;

pub struct TransformComponent {
    pub position: Vect3,
    pub forward: Vect3,
    pub up: Vect3,
    pub scale: Vect3,
}

impl TransformComponent {
    pub fn default() -> Self {
        TransformComponent {
            position: [0.0, 0.0, 0.0].into(),
            forward: [1.0, 0.0, 0.0].into(),
            up: [0.0, 0.0, 1.0].into(),
            scale: [1.0, 1.0, 1.0].into(),
        }
    }

    pub fn with_position(mut self, position: Vect3) -> Self {
        self.position = position;
        self
    }

    pub fn with_forward(mut self, forward: Vect3) -> Self {
        self.forward = forward;
        self
    }

    pub fn with_up(mut self, up: Vect3) -> Self {
        self.up = up;
        self
    }

    pub fn with_scale(mut self, scale: Vect3) -> Self {
        self.scale = scale;
        self
    }

    pub fn get_transform_matrix(&self) -> Mat4 {
        Mat4::identity()
            .translate(self.position)
            .scale(self.scale)
            .rotate_with_dir_and_up(self.forward, self.up)
    }
}