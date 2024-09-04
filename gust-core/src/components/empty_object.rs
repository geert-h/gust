use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use gust_math::matrices::mat4::Mat4;
use crate::primitives::mesh::Mesh;
use crate::scene::scene_tree::GameTreeObject;
use crate::systems::game::Game;

pub struct EmptyObject {
    // Empty object
    // This object is used to represent an empty object in the system
}

impl GameTreeObject for EmptyObject {
    fn update(&mut self, _game: &mut Game) {
        // Do nothing
    }

    fn render(&self) {
        // Do nothing
    }

    fn get_model_matrix(&self) -> Mat4 {
        Mat4::identity()
    }

    fn get_mesh(&self) -> Rc<Mesh> {
        Rc::new(Mesh::new())
    }

    fn get_texture(&self, _display: Display<WindowSurface>) -> glium::texture::Texture2d {
        let image = image::RgbaImage::new(1, 1);
        let dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        glium::texture::Texture2d::new(&_display, image).unwrap()
    }
}