use glium::{Display, Texture2d};
use glium::glutin::surface::WindowSurface;
use glium::texture::RawImage2d;

use gust_math::matrices::matrix::Matrix;
use gust_math::vectors::vect::Vect;

use crate::data::color::Color;
use crate::data::mesh::{from_wavefront_object, Mesh};
use crate::parsers::wavefront_object_parser;

pub struct GameObject {
    pub id: u32,
    pub name: String,
    pub position: Vect,
    pub rotation: Matrix,
    pub scale: f32,
    pub mesh: Mesh,
    pub color: Color,
    pub texture: Texture2d,
}

impl GameObject {
    pub fn new(id: u32, name: String, position: Vect, rotation: Matrix, scale: f32, mesh: Mesh, color: Color, texture: Texture2d) -> Self {
        GameObject {
            id,
            name,
            position,
            rotation,
            scale,
            mesh,
            color,
            texture,
        }
    }

    pub fn init(display: &Display<WindowSurface>) -> Self {
        let wavefront_object = wavefront_object_parser::parse_wavefront_object("C:\\Users\\Geert\\source\\repos\\Personal\\gust\\resources\\assets\\objects\\BalKubus.obj");
        let mesh = from_wavefront_object(wavefront_object);

        let position = Vect::from_slice(&[0.0, 0.0, 0.0]);
        let rotation = Matrix::identity(3);
        let scale = 1.0;
        let color = Color::new(1.0, 1.0, 1.0, 1.0);

        let image = image::load(std::io::Cursor::new(&include_bytes!("../../../resources/assets/BallRender.png")), image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, image).unwrap();

        GameObject::new(0, "cube".to_string(), position, rotation, scale, mesh, color, texture)
    }
}
