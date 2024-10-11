use gust_systems::game::Game;

fn main() {
    test();
    //Game::new().run();
}

fn test() {
    let mut world = World::new();
    let icarus_entity = world.new_entity();
    world.add_component_to_entity(icarus_entity, Name("Icarus"));
    world.add_component_to_entity(icarus_entity, Health(-10));

    let prometheus_entity = world.new_entity();
    world.add_component_to_entity(prometheus_entity, Name("Prometheus"));
    world.add_component_to_entity(prometheus_entity, Health(100));

    let zeus_entity = world.new_entity();
    world.add_component_to_entity(zeus_entity, Name("Zeus"));

    let mut healths = world.borrow_component_vec_mut::<Health>().unwrap();
    let mut names = world.borrow_component_vec_mut::<Name>().unwrap();
    let zip = healths.iter_mut().zip(names.iter_mut());
    let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

    for (health, name) in iter {
        if health.0 < 0 {
            println!("{} has perished", name.0)
        }

        if name.0 == "Perseus" && health.0 <= 0 {
            *health = Health(100);
        }
    }
}
