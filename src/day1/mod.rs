use super::util::read;
use std::cmp::Ord;
use std::cmp::Ordering;

struct Elf {
    index: i32,
    calories: Vec<i32>,
}

impl Elf {
    pub fn sum(&self) -> i32 {
        return self.calories.iter().sum();
    }
}


fn parse(input: Vec<String>) -> Vec<Elf> {
    let mut buffer: Vec<i32> = Vec::new();
    let mut elfs: Vec<Elf> = Vec::new();
    let mut i = 1;
    for item in input {
        if item == "" {
            let elf = Elf {
                index: i,
                calories: buffer.clone(),
            };
            elfs.push(elf);
            buffer.clear();
            i = i + 1;
        } else {
            let int: i32 = item.parse().unwrap();
            buffer.push(int);
        }
    }
    let elf = Elf {
        index: i,
        calories: buffer.clone(),
    };
    elfs.push(elf);
    return elfs;
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: i32 = self.sum();
        let b: i32 = other.sum();
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

pub fn run_part_1(file_name: &str) -> i32 {
    let lines = read(file_name).expect("error reading file");
    let elfs = parse(lines);
    let elf: &Elf = elfs.iter().max().unwrap();
    return elf.calories.iter().sum();
}

pub fn run_part_2(file_name: &str) -> i32 {
    let lines = read(file_name).expect("error reading file");
    let mut elfs = parse(lines);

    elfs.sort();
    elfs.reverse();

    return elfs.iter().take(3).map(|e: &Elf| e.sum()).sum();
}

#[cfg(test)]
mod tests {
    use super::run_part_1;
    use super::run_part_2;

    #[test]
    fn part_1_small_set() {
        assert_eq!(24000, run_part_1("src/day1/test"))
    }

    #[test]
    fn part_1_large_set() {
        assert_eq!(71023, run_part_1("src/day1/input"))
    }

    #[test]
    fn part_2_small_set() {
        assert_eq!(45000, run_part_2("src/day1/test"))
    }

    #[test]
    fn part_2_large_set() {
        assert_eq!(206289, run_part_2("src/day1/input"))
    }
}
