#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_develiry_time() {
        let world = create_initial_world();

        if let Location(location) = world.get_location("factory".to_string()) {

        }



    }

    fn create_initial_world() -> World {
        let mut world = World::new();

        for location in list_locations() {
            world.add_location(location);
        }

        for road in list_roads() {
            world.add_road(road);
        }

        for transporter in list_transporters() {
            world.add_transporter(transporter);
        }


        world
    }

    fn list_locations() -> Vec<Location> {
        vec![
            Location { name: "factory".to_string(), kind: LocationKind::factory },
            Location { name: "A".to_string(), kind: LocationKind::destination },
            Location { name: "B".to_string(), kind: LocationKind::destination },
            Location { name: "port".to_string(), kind: LocationKind::port },
        ]
    }

    fn list_roads() -> Vec<Road>{
        vec![
            Road{
                start: "factory".to_string(), end: "port".to_string(),
                delay: 1, road_type: RoadType::Ground},
            Road{
                start: "port".to_string(), end: "A".to_string(),
                delay: 4, road_type: RoadType::Sea},
            Road{
                start: "factory".to_string(), end: "B".to_string(),
                delay: 5, road_type: RoadType::Ground
            }
        ]
    }

    fn list_transporters() -> Vec<Transporter> {
        vec![
            Transporter::Truck {name: "truck 1".to_string(), start: "factory".to_string()},
            Transporter::Truck {name: "truck 2".to_string(), start: "factory".to_string()},
            Transporter::Boat {name: "boat 1".to_string(), start: "port".to_string()},
        ]
    }


}

#[derive(Clone,Debug)]
struct Location {
    name : String,
    kind: LocationKind,
}

#[derive(Clone,Debug)]
enum LocationKind {
    factory,
    destination,
    port
}

struct Road {
    start: String,
    end: String,
    delay: u32,
    road_type: RoadType,
}

enum RoadType {
    Ground,
    Sea
}

enum Transporter {
    Truck {name: String, start: String},
    Boat {name: String, start: String},
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

    fn add_road(&mut self, road: Road) {

    }

    fn add_transporter(&mut self, transporter: Transporter) {

    }

    fn get_location(&self, name: String) -> Result<Location, Error>{
        for location in self.locations {
            return Ok(location);
        }
        Err(Error { })
    }
}

struct Error {

}
