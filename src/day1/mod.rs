use std::cmp::Ord;
use std::cmp::Ordering;
use super::util::read;

struct Elf {
    calories: Vec<i32>,
}

fn parse(input: Vec<String>) -> Vec<Elf> {
    let mut buffer: Vec<i32> = Vec::new();
    let mut elfs: Vec<Elf> = Vec::new();

    for item in input {
        if item == "" {
            let elf = Elf {
                calories: buffer.clone(),
            };
            elfs.push(elf);
            buffer.clear()
        } else {
            let int: i32 = item.parse().unwrap();
            buffer.push(int)
        }
    }
    return elfs;
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: i32 = self.calories.to_owned().into_iter().sum();
        let b: i32 = other.calories.to_owned().into_iter().sum();
        return a.cmp(&b);
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}
impl Eq for Elf {}

fn analyse(elfs: Vec<Elf>) -> i32 {
    let elf: Elf = elfs.into_iter().max().unwrap();
    return elf.calories.into_iter().sum();
}

pub fn test() {
    let file_name = "src/day1/test";
    let lines = read(file_name).expect("error reading file");
    let elfs = parse(lines);
    println!("{}", analyse(elfs))
}

pub fn run() {
    let file_name = "src/day1/input";
    let lines = read(file_name).expect("error reading file");
    let elfs = parse(lines);
    println!("{}", analyse(elfs))
}
