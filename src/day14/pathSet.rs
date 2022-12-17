use std::cell::RefCell;

use crate::day14::{Coordinates, Path};

pub struct PathSet(pub(crate) RefCell<Vec<Path>>);

impl PathSet {
    pub fn parse(input: &[String]) -> PathSet {
        let paths: Vec<Path> = input
            .iter()
            .map(|line| {
                line.split("->")
                    .map(|s| s.trim())
                    .map(|s| {
                        let (xs, ys) = s.split_once(',').unwrap();
                        let x: i32 = xs.parse().unwrap();
                        let y: i32 = ys.parse().unwrap();
                        Coordinates { x, y }
                    })
                    .collect()
            })
            .collect();
        PathSet(RefCell::from(paths))
    }

    pub fn normalize(&self) {
        let x_min = self.flat_min(|p| p.x);
        self.0
            .borrow_mut()
            .iter_mut()
            .for_each(|path| path.iter_mut().for_each(|c| c.x -= x_min));
    }

    pub fn flat_max(&self, fun: fn(&Coordinates) -> i32) -> i32 {
        self.0
            .borrow()
            .iter()
            .flat_map(|v| v.iter().map(fun))
            .max()
            .unwrap()
    }

    pub fn flat_min(&self, fun: fn(&Coordinates) -> i32) -> i32 {
        self.0
            .borrow()
            .iter()
            .flat_map(|v| v.iter().map(fun))
            .min()
            .unwrap()
    }
}
