use glium::implement_uniform_block;

#[derive(Copy, Clone)]
pub struct LightsBlock {
    pub(crate) light_positions: [[f32; 3]; 5],
    pub(crate) _padding: [f32; 5],
    pub(crate) light_colors: [[f32; 3]; 5],
}
implement_uniform_block!(LightsBlock, light_positions, light_colors);
