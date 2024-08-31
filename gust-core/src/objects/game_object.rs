use std::path::Path;

use glium::Display;
use glium::glutin::surface::WindowSurface;
use image::RgbaImage;

use gust_math::matrices::mat4::Mat4;

use crate::objects::intermediaries::wavefront_object::WavefrontObject;
use crate::primitives::mesh::Mesh;

pub struct GameObject {
    pub id: u32,
    pub name: String,
    pub mesh: Mesh,
    pub image: RgbaImage,
    pub object_to_parent: Mat4,
}

impl GameObject {
    pub fn new(id: u32, name: String, image: RgbaImage, mesh: Mesh, object_to_parent: Mat4) -> Self {
        GameObject {
            id,
            name,
            mesh,
            image,
            object_to_parent,
        }
    }

    pub fn init_floor_object() -> Self {
        let wavefront_object = WavefrontObject::parse(Path::new("./resources/assets/objects/floor.obj"));
        let mesh = Mesh::from_wavefront(wavefront_object);
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/wood.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();

        GameObject::new(0, "floor".to_string(), image, mesh, Mat4::identity())
    }

    pub fn get_texture(&self, display: &Display<WindowSurface>) -> glium::texture::Texture2d {
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&self.image.clone().into_raw(), self.image.dimensions());
        glium::texture::Texture2d::new(display, image).unwrap()
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        self.object_to_parent.to_slices()
    }
}
