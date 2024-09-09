use gust_ecs::component::Component::TransformComponent;
use gust_ecs::component::ComponentType::TransformType;
use gust_ecs::component::Transform;
use gust_hierarchy::node::propagate_transform;
use gust_hierarchy::world::World;
use gust_math::matrices::mat4::Mat4;

fn main() {
    // Game::new().run();
    // test_gust_hierarchy();
    test();
}

fn test() {
    let mut world = World::new();

    // Create entities
    let parent = world.create_entity();
    let child = world.create_entity();

    // Set up the scene tree
    world.scene_tree.set_parent(parent, child);

    // Add components
    world.component_storage.add_component(parent, TransformComponent(Transform(Mat4::identity().scale([1.0, 2.0, 3.0].into()))));
    world.component_storage.add_component(child, TransformComponent(Transform(Mat4::identity().scale([1.0, 2.0, 3.0].into()))));

    // Propagate transforms from parent to child
    propagate_transform(&mut world);

    // Get the child's global position
    if let Some(TransformComponent(child_transform)) = world.component_storage.get_component(child, TransformType) {
        println!("Child's global position: {:?}", child_transform.0);
    }
}