use crate::day14::Coordinates;

pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    pub fn find_direction(a: &Coordinates, b: &Coordinates) -> Direction {
        match (a, b) {
            _ if a.x < b.x && a.y == b.y => Direction::EAST,
            _ if a.x > b.x && a.y == b.y => Direction::WEST,
            _ if a.x == b.x && a.y < b.y => Direction::SOUTH,
            _ if a.x == b.x && a.y > b.y => Direction::NORTH,
            _ => panic!(),
        }
    }
}
