use std::rc::Rc;
use gust_core::systems::game::Game;
use gust_math::matrices::mat4::Mat4;

pub trait NodeTrait {
    fn update(&mut self, game: &mut Game);

    fn render(&self);

    fn get_world_transform(&self) -> Mat4;

    fn add_child(&mut self, child: Rc<Self>);

    fn remove_child(&mut self, child: Rc<Self>);
}