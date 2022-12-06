use std::{cell::RefCell, collections::HashMap, hash::Hash};

use crate::util;

type Crate = char;

struct Move {
    amount: usize,
    from: i32,
    to: i32,
}

impl Move {
    fn parse(line: &str) -> Move {
        let arr: Vec<&str> = line.split_ascii_whitespace().collect();

        let amount: usize = arr[1].parse().unwrap();
        let from: i32 = arr[3].parse().unwrap();
        let to: i32 = arr[5].parse().unwrap();

        Move { amount, from, to }
    }
}
type Stack = HashMap<i32, Vec<Crate>>;

trait IStack {
    fn apply(&mut self, r#move: &Move);
}

impl IStack for Stack {
    fn apply(&mut self, r#move: &Move) {
        let from = &mut self.get_mut(&r#move.from).unwrap();
        let mut to_move = from.as_slice()[from.len() - r#move.amount..].to_vec();
        let to = &mut self.get_mut(&r#move.to).unwrap();
        to.append(&mut to_move);
    }

}

fn apply(map: &mut Stack, r#move: &Move) {
    let from = &mut map.get_mut(&r#move.from).unwrap();
    let mut to_move = from.as_slice()[from.len() - r#move.amount..].to_vec();
    let to = &mut map.get_mut(&r#move.to).unwrap();
    to.append(&mut to_move);
}

fn init(input: &[String]) -> Vec<Move> {
    input
        .iter()
        .filter(|s| s.starts_with("move"))
        .map(|s| Move::parse(s))
        .collect()
}

fn stack_size(input: &[String]) -> usize {
       for line in input  {
           if line.starts_with(" 1") {
               return line
               .trim()
               .               chars()
               .               last()
               .unwrap()
               .to_digit(10)
               .unwrap() 
               as usize;
           }
       }
       return 0;
}

// fn init_vec_stack(input: &[String]) -> Stack {
//     return ;

// }

// fn init_stack(input: &[String]) -> Stack {

// }

fn run() {
    let input = util::read("src/day3/test");
    let moves: Vec<Move> = init(&input);

    let mut map: Stack = HashMap::new();

    for r#move in moves {

        map.apply(&r#move);
        apply(&mut map, &r#move);
    }
}
