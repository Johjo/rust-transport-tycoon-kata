#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_world_with_locations() {
        assert_world_contains_location(vec![],
                                       String::from("World { locations: [] }"));
        assert_world_contains_location(vec![Location::Factory("factory".to_string())],
                                       String::from("World { locations: [Factory(\"factory\")] }"));
        assert_world_contains_location(vec![   Location::Factory("factory".to_string()),
                                                     Location::Destination(String::from("A"))],
                                       String::from("World { locations: [Factory(\"factory\"), Destination(\"A\")] }"));
        assert_world_contains_location(
            vec![
                Location::Factory("factory".to_string()),
                Location::Destination(String::from("A")),
                Location::Destination(String::from("B")),
                Location::Port(String::from("port")),
            ],
            String::from("World { locations: [\
                                                Factory(\"factory\"), \
                                                Destination(\"A\"), \
                                                Destination(\"B\"), \
                                                Port(\"port\")] \
                                            }"));
    }

    fn assert_world_contains_location(locations: Vec<Location>, expected: String) {
        let mut world = World::new();
        for location in locations {
            world.add_location(location);
        }
        assert_eq!(format!("{:?}", world), expected);
    }

    #[test]
    fn should_add_road_to_world() {
        let world = create_world();
    }

    fn create_world() -> World {
        let mut world = World::new();

        let locations = vec![
            Location::Factory("factory".to_string()),
            Location::Destination(String::from("A")),
            Location::Destination(String::from("B")),
            Location::Port(String::from("port")),
        ];

        for location in locations {
            world.add_location((location));
        }

        world
    }

}

#[derive(Clone,Debug)]
enum Location {
    Factory(String),
    Destination(String),
    Port(String)
}

#[derive(Debug)]
struct World {
    locations: Vec<Location>,
}

impl World {
    fn new() -> World {
        World {
            locations: Vec::new(),
        }
    }

    fn add_location(&mut self, location: Location) {
        self.locations.push(location);
    }

}