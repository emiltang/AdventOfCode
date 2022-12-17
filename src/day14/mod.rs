use pathSet::PathSet;

use crate::day14::world::World;
use crate::util;

mod direction;
mod element;
mod pathSet;
mod row;
mod world;

#[derive(Debug)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

type Path = Vec<Coordinates>;

#[cfg(test)]
mod tests {}

pub fn print_filled(file: &str) {
    let lines = util::read(file);
    let path_set = PathSet::parse(&lines);
    let map = World::new(&path_set);
    println!("{}", map)
}
