use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use image::RgbaImage;
use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;
use crate::primitives::mesh::Mesh;
use crate::scene::scene_tree::GameTreeObject;
use crate::systems::game::Game;

pub struct FloorObject {
    pub position: Vect3,
    pub direction: Vect3,
    pub up: Vect3,
    pub mesh: Rc<Mesh>,
    pub image: Rc<RgbaImage>,
}

impl FloorObject {
    pub fn new(position: Vect3, up: Vect3, direction: Vect3, mesh: Rc<Mesh>, image: Rc<RgbaImage>) -> Self {
        FloorObject {
            position,
            direction,
            up,
            mesh,
            image,
        }
    }
}

impl GameTreeObject for FloorObject {
    fn update(&mut self, _game: &mut Game) {
        // Do nothing
    }

    fn render(&self) {
        // Do nothing
    }

    fn get_model_matrix(&self) -> Mat4 {
        Mat4::identity().translate(self.position).rotate_with_dir_and_up(self.direction, self.up)
    }

    fn get_mesh(&self) -> Rc<Mesh> {
        self.mesh.clone()
    }

    fn get_texture(&self, display: Display<WindowSurface>) -> glium::texture::Texture2d {
        let image = match Rc::try_unwrap(self.image.clone()) {
            Ok(image) => image,
            Err(rc_image) => (*rc_image).clone(),
        };
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.clone().into_raw(), self.image.dimensions());
        glium::texture::Texture2d::new(&display, image).unwrap()
    }
}
