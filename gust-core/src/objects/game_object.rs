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
    pub transformation: Mat4,
}

impl GameObject {
    pub fn new(id: u32, name: String, image: RgbaImage, mesh: Mesh, transformation: Mat4) -> Self {
        GameObject {
            id,
            name,
            mesh,
            image,
            transformation,
        }
    }

    pub fn init() -> Self {
        let wavefront_object = WavefrontObject::parse("C:\\Users\\Geert\\source\\repos\\Personal\\gust\\resources\\assets\\objects\\BalKubus.obj");
        let mesh = Mesh::from_wavefront(wavefront_object);

        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/BallRender.png")), image::ImageFormat::Png).unwrap().to_rgba8();

        GameObject::new(0, "cube".to_string(), image, mesh, Mat4::identity())
    }

    pub fn init_floor_object() -> Self {
        let wavefront_object = WavefrontObject::parse("C:\\Users\\Geert\\source\\repos\\Personal\\gust\\resources\\assets\\objects\\Floor.obj");
        let mesh = Mesh::from_wavefront(wavefront_object);
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/BallRender.png")), image::ImageFormat::Png).unwrap().to_rgba8();

        GameObject::new(0, "floor".to_string(), image, mesh, Mat4::identity())
    }

    pub fn get_texture(&self, display: &Display<WindowSurface>) -> glium::texture::Texture2d {
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&self.image.clone().into_raw(), self.image.dimensions());
        glium::texture::Texture2d::new(display, image).unwrap()
    }
}
