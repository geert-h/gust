#[macro_use]
extern crate glium;

use std::fmt::Display;

use glium::{Frame, Surface, VertexBuffer};
use winit::dpi::PhysicalPosition;
use winit::event::Event::WindowEvent;
use winit::event::KeyEvent;
use winit::window::CursorGrabMode;

use gust_core::data::game::Game;
use gust_core::data::mesh::*;
use gust_core::data::vertex::Vertex;
use gust_core::parsers::wavefront_object_parser;
use gust_math::vect::Vect;

fn main() {
    let position = [0.0, 0.0, 5.0];
    let direction = [0.0f32, 0.0, -1.0];

    let frac_shader_string = include_str!("../../resources/shaders/frac.glsl");
    let vert_shader_string = include_str!("../../resources/shaders/vert.glsl");

    let wavefront_object = wavefront_object_parser::parse_wavefront_object("C:\\Users\\Geert\\source\\repos\\Personal\\gust\\resources\\assets\\objects\\monkey.obj");
    let object = from_wavefront_object(wavefront_object);

    let mut game = Game::new();

    game.player.direction = Vect::from_slice(&direction);
    game.player.position = Vect::from_slice(&position);

    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Gust")
        .build(&event_loop);

    window.set_cursor_grab(CursorGrabMode::Locked)
        .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Confined))
        .unwrap();

    window.set_cursor_visible(false);

    let mut t: f32 = 0.0;

    let flattened_triangles: Vec<Vertex> = object.triangles
        .iter()
        .flat_map(|triangle| triangle
            .iter()
            .cloned()
        )
        .collect();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_buffer = VertexBuffer::new(&display, &flattened_triangles).unwrap();
    let mut mouse_position = PhysicalPosition::new(400.0, 240.0);
    event_loop
        .run(move |event, window_target| {
            match event {
                WindowEvent { event: window_event, .. } => match window_event {
                    winit::event::WindowEvent::KeyboardInput { event: KeyEvent { logical_key: key, state, .. }, .. } => {
                        match state {
                            winit::event::ElementState::Pressed => {
                                game.handle_keyboard_input(key);
                            }
                            winit::event::ElementState::Released => {
                                game.handle_key_release(key);
                            }
                        }
                    }
                    winit::event::WindowEvent::CursorMoved { position: new_position, .. } => {
                        mouse_position = new_position;
                        window.set_cursor_position(PhysicalPosition::new(400.0, 240.0)).unwrap();
                        game.handle_mouse_input(mouse_position);
                        mouse_position = PhysicalPosition::new(400.0, 240.0);
                    }
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        t += 0.02;
                        game.handle_mouse_input(mouse_position);
                        game.update();

                        let program = glium::Program::from_source(
                            &display,
                            vert_shader_string,
                            frac_shader_string,
                            None,
                        )
                            .unwrap();

                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            backface_culling:
                            glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                            ..Default::default()
                        };

                        let mut target = display.draw();
                        target.clear_color_and_depth((0.3, 0.3, 0.4, 1.0), 1.0);
                        target
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &get_uniforms(game.player.position.to_vec().try_into().unwrap(), game.player.direction.to_vec().try_into().unwrap(), t, &target),
                                &params,
                            )
                            .unwrap();

                        target.finish().unwrap();
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            };
        })
        .unwrap();
}

fn get_uniforms(position: [f32; 3], direction: [f32; 3], t: f32, target: &Frame) -> impl glium::uniforms::Uniforms {
    let light = [1.4, 0.4, -0.7f32];

    let view = view_matrix(&position, &direction, &[0.0, 1.0, 0.0]);

    let perspective = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = std::f32::consts::PI / 3.0;
        let z_far = 1024.0;
        let z_near = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (z_far + z_near) / (z_far - z_near), 1.0],
            [0.0, 0.0, -(2.0 * z_far * z_near) / (z_far - z_near), 0.0],
        ]
    };

    uniform! {
        perspective: perspective,
        model: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ],
        u_light: light,
        view : view,
    }
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    // let f = Vect::from_slice(position).normalize();
    //
    // let s = f.cross(&Vect::from_slice(up).normalize()).expect("Error while taking cross product").normalize();
    //
    // let u = s.cross(&f).expect("Error while taking cross product").normalize();
    //
    // let p = [
    //     -position[0] * s[0] - position[1] * s[1] - position[2] * s[2],
    //     -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
    //     -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    // ];
    //
    // let p = Vect::from_slice(&p);
    //
    // [
    //     [s[0], u[0], -f[0], 0.0],
    //     [s[1], u[1], -f[1], 0.0],
    //     [s[2], u[2], -f[2], 0.0],
    //     [p[0], p[1], p[2], 1.0],
    // ]


    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };
    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}