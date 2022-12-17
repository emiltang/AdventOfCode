use std::cell::RefCell;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::day14::direction::Direction;
use crate::day14::element::Element;
use crate::day14::pathSet::PathSet;
use crate::day14::row::Row;

use super::{Coordinates, Path};

pub struct World {
    height: i32,
    width: i32,
    data: RefCell<Vec<Element>>,
}

impl World {
    fn get_line(&self, y: i32) -> Row {
        let mut result: Row = Row(Vec::new());

        for x in 0..self.width {
            let element = self.get_element(x, y);
            result.0.push(element);
        }
        result
    }

    pub fn new(path_set: &PathSet) -> World {
        path_set.normalize();
        let height = path_set.flat_max(|p| p.y) + 1;
        let width = path_set.flat_max(|p| p.x) - path_set.flat_min(|p| p.x) + 1;
        let data = RefCell::new(vec![Element::EMPTY; (height * width) as usize]);
        let map = World {
            height,
            width,
            data,
        };
        map.fill(path_set);
        map
    }

    fn get_element(&self, x: i32, y: i32) -> Element {
        let index = x * self.width + y;
        let vec = self.data.borrow();
        vec[index as usize].clone()
    }

    fn set_rock(&self, x: i32, y: i32) {
        let mut vec = self.data.borrow_mut();
        let index = x * self.width + y;
        vec[index as usize] = Element::ROCK;
    }

    fn draw_path(&self, from: &Coordinates, to: &Coordinates) {
        match Direction::find_direction(from, to) {
            Direction::NORTH => self.draw_vertical(to, from),
            Direction::SOUTH => self.draw_vertical(from, to),
            Direction::EAST => self.draw_horizontal(from, to),
            Direction::WEST => self.draw_horizontal(to, from),
        }
    }

    pub fn fill(&self, paths: &PathSet) {
        for path in paths.0.borrow().iter() {
            for (from, to) in path.iter().tuple_windows() {
                self.draw_path(from, to)
            }
        }
    }
    fn draw_vertical(&self, from: &Coordinates, to: &Coordinates) {
        for y in from.y..=to.y {
            self.set_rock(from.x, y)
        }
    }

    fn draw_horizontal(&self, from: &Coordinates, to: &Coordinates) {
        for x in from.x..=to.x {
            self.set_rock(x, from.y)
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;
        for i in 0..self.width {
            write!(f, "{}", i)?;
        }
        writeln!(f)?;
        for h in 0..self.height {
            writeln!(f, "{}{}", h, self.get_line(h))?;
        }
        Ok(())
    }
}
