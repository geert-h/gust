use glium::implement_uniform_block;

#[derive(Copy, Clone)]
pub struct LightsBlock {
    pub light_positions: [[f32; 3]; 5],
    pub _padding: [f32; 5],
    pub light_colors: [[f32; 3]; 5],
}
implement_uniform_block!(LightsBlock, light_positions, light_colors);
