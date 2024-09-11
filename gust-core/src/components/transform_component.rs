use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;

pub struct TransformComponent {
    pub position: Vect3,
    pub forward: Vect3,
    pub up: Vect3,
    pub scale: Vect3,
}

impl TransformComponent {
    pub fn get_transform_matrix(&self) -> Mat4 {
        Mat4::identity()
            .translate(self.position)
            .scale(self.scale)
            .rotate_with_dir_and_up(self.forward, self.up)
    }
}