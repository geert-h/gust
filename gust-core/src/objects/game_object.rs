use glium::Display;
use glium::glutin::surface::WindowSurface;
use image::RgbaImage;

use gust_math::matrices::mat3::Mat3;
use gust_math::vectors::vect3::Vect3;

use crate::data::color::Color;
use crate::data::mesh::Mesh;
use crate::parsers::wavefront_parser;

pub struct GameObject {
    pub id: u32,
    pub name: String,
    pub position: Vect3,
    pub rotation: Mat3,
    pub scale: f32,
    pub mesh: Mesh,
    pub color: Color,
    image: RgbaImage,
}

impl GameObject {
    pub fn new(id: u32, name: String, position: Vect3, rotation: Mat3, scale: f32, mesh: Mesh, color: Color, image: RgbaImage) -> Self {
        GameObject {
            id,
            name,
            position,
            rotation,
            scale,
            mesh,
            color,
            image,
        }
    }

    pub fn init(v: f32) -> Self {
        let wavefront_object = wavefront_parser::parse_wavefront_object("C:\\Users\\Geert\\source\\repos\\Personal\\gust\\resources\\assets\\objects\\BalKubus.obj");
        let mesh = Mesh::from_wavefront(wavefront_object, v);

        let position = Vect3::from_slice(&[0.0, 0.0, 0.0]);
        let rotation = Mat3::identity();
        let scale = 1.0;
        let color = Color::new(1.0, 1.0, 1.0, 1.0);

        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/BallRender.png")), image::ImageFormat::Png).unwrap().to_rgba8();

        GameObject::new(0, "cube".to_string(), position, rotation, scale, mesh, color, image)
    }

    pub fn init_floor_object() -> Self {
        let mesh = Mesh::construct_floor_mesh();
        let position = Vect3::from_slice(&[0.0, 0.0, 0.0]);
        let rotation = Mat3::identity();
        let scale = 1.0;
        let color = Color::new(1.0, 1.0, 1.0, 1.0);

        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/BallRender.png")), image::ImageFormat::Png).unwrap().to_rgba8();

        GameObject::new(0, "floor".to_string(), position, rotation, scale, mesh, color, image)
    }

    pub fn get_texture(&self, display: &Display<WindowSurface>) -> glium::texture::Texture2d {
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&self.image.clone().into_raw(), self.image.dimensions());
        glium::texture::Texture2d::new(display, image).unwrap()
    }
}
