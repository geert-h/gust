use std::rc::Rc;

use glium::Display;
use glium::glutin::surface::WindowSurface;
use image::RgbaImage;

use gust_math::matrices::mat4::Mat4;

use crate::primitives::mesh::Mesh;

pub struct GameObject {
    pub id: u32,
    pub name: String,
    pub mesh: Rc<Mesh>,
    pub image: Rc<RgbaImage>,
    pub object_to_parent: Mat4,
}

impl GameObject {
    pub fn new(id: u32, name: String, image: Rc<RgbaImage>, mesh: Rc<Mesh>, object_to_parent: Mat4) -> Self {
        GameObject {
            id,
            name,
            mesh,
            image,
            object_to_parent,
        }
    }

    pub fn get_texture(&self, display: &Display<WindowSurface>) -> glium::texture::Texture2d {
        let image = match Rc::try_unwrap(self.image.clone()) {
            Ok(image) => image,
            Err(rc_image) => (*rc_image).clone(),
        };
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.clone().into_raw(), self.image.dimensions());
        glium::texture::Texture2d::new(display, image).unwrap()
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        self.object_to_parent.to_slices()
    }
}
