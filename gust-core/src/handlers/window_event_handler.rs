// use glium::glutin::surface::WindowSurface;
// use winit::event::Event::WindowEvent;
//
// pub struct WindowEventHandler {
//     pub window: winit::window::Window,
//     pub display : Display<WindowSurface>,
// }
//
// impl WindowEventHandler {
//     pub fn new(window: winit::window::Window) -> Self {
//         Self {
//             window
//         }
//     }
//
//     pub fn handle_window_event(&mut self, event: winit::event::Event<()>) {
//             match event {
//                 WindowEvent { event: window_event, .. } => match window_event {
//                     winit::event::WindowEvent::KeyboardInput { event : KeyEvent { logical_key : key, ..}, .. } => new_handle_inputs(&mut direction, &mut position, key),
//                     winit::event::WindowEvent::CloseRequested => window_target.exit(),
//                     winit::event::WindowEvent::Resized(window_size) => {
//                         display.resize(window_size.into());
//                     }
//                     winit::event::WindowEvent::RedrawRequested => {
//                         t += 0.02;
//
//                         let program = glium::Program::from_source(
//                             &display,
//                             vert_shader_string,
//                             frac_shader_string,
//                             None,
//                         )
//                             .unwrap();
//
//                         let params = glium::DrawParameters {
//                             depth: glium::Depth {
//                                 test: glium::draw_parameters::DepthTest::IfLess,
//                                 write: true,
//                                 ..Default::default()
//                             },
//                             backface_culling:
//                             glium::draw_parameters::BackfaceCullingMode::CullClockwise,
//                             ..Default::default()
//                         };
//
//                         let mut target = display.draw();
//                         target.clear_color_and_depth((0.3, 0.3, 0.4, 1.0), 1.0);
//                         target
//                             .draw(
//                                 &vertex_buffer,
//                                 &indices,
//                                 &program,
//                                 &get_uniforms(position, direction, t, &target),
//                                 &params,
//                             )
//                             .unwrap();
//
//                         target.finish().unwrap();
//                     },
//                     winit::event::WindowEvent::CursorMoved { position, .. } => {
//                         handle_mouse_input(position, &mut direction, &mut mouse_position);
//                         window.set_cursor_position(PhysicalPosition::new(400.0, 240.0)).unwrap();
//                         mouse_position = PhysicalPosition::new(400.0, 240.0);
//                     },
//                     _ => (),
//                 },
//                 winit::event::Event::AboutToWait => {
//                     window.request_redraw();
//                 },
//                 _ => (),
//             };
//         }
// }