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
    let parent = world.spawn();
    let child = world.spawn();

    // Set up the scene tree
    world.set_parent(parent, child);

    // Add components
    world.add_component(parent, TransformComponent(Transform(Mat4::identity())));
    world.add_component(child, TransformComponent(Transform(Mat4::identity())));

    // Propagate transforms from parent to child
    propagate_transform(&mut world);

    // Get the child's global position
    if let Some(TransformComponent(child_transform)) = world.get_component(child, TransformType) {
        println!("Child's global position: {:?}", child_transform.0);
    }

    if let Some(TransformComponent(parent_transform)) = world.get_component(parent, TransformType) {
        println!("Parent's global position: {:?}", parent_transform.0);
    }
}