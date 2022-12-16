use std::cell::RefCell;

use itertools::Itertools;

use super::{Pair, Path};

struct Map {
    height: i32,
    width: i32,
    data: RefCell<Vec<bool>>,
}

impl Map {
    fn new(height: i32, width: i32) -> Map {
        Map {
            height,
            width,
            data: RefCell::new(vec![false; (height * width) as usize]),
        }
    }

    fn set(&self, x: i32, y: i32) {
        let mut vec = self.data.borrow_mut();
        let index = x * self.width + y;
        vec[index as usize] = true;
    }

    fn draw_path(&self, from: &Pair, to: &Pair) {}

    fn fill(&self, paths: &Vec<Path>) {
        for path in paths {
            for (from, to) in path.iter().tuples() {
                self.draw_path(from, to)
            }
        }
    }
}