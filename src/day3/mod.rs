use std::collections::HashSet;

use crate::util::read;

#[derive(Debug, PartialEq)]
struct Rucksack {
    index: u32,
    compartmen_one: Vec<char>,
    compartmen_two: Vec<char>,
}

impl Rucksack {
    fn parse_vec(input: &Vec<String>) -> Vec<Rucksack> {
        return input
            .iter()
            .enumerate()
            .map(|(i, s)| Rucksack::parse(s, i))
            .collect();
    }

    fn parse(input: &String, i: usize) -> Rucksack {
        let first = &input[..input.len() / 2];
        let second = &input[input.len() / 2..input.len()];
        return Rucksack {
            index: i as u32,
            compartmen_one: first.chars().collect(),
            compartmen_two: second.chars().collect(),
        };
    }

    fn common_priority_sum(&self) -> i32 {
        let common_priorities = common_priorities(&self.compartmen_one, &self.compartmen_two);
        return common_priorities.iter().map(|c| c.priority()).sum();
    }

    fn elements(&self) -> Vec<char> {
        return self
            .compartmen_one
            .iter()
            .copied()
            .chain(self.compartmen_two.iter().copied())
            .collect();
    }
}

fn common_priorities(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut vec: Vec<char> = a
        .into_iter()
        .filter(|c| b.contains(c))
        .map(|c| *c)
        .collect();
    vec.sort();
    vec.dedup();
    return vec;
}

fn common_priorities_three(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>) -> Vec<char> {
    let a_set: HashSet<char> = HashSet::from_iter(a.clone());
    let b_set: HashSet<char> = HashSet::from_iter(b.clone());
    let c_set: HashSet<char> = HashSet::from_iter(c.clone());

    return a_set
        .into_iter()
        .filter(|d| b_set.contains(d) && c_set.contains(d))
        .collect();
}

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for char {
    fn priority(&self) -> i32 {
        return match self {
            lower if char::is_ascii_lowercase(lower) => (*lower as i32) - 96,
            upper if char::is_ascii_uppercase(upper) => (*upper as i32) - 38,
            _ => panic!("invalid input"),
        };
    }
}

struct RucksackGroup {
    one: Rucksack,
    two: Rucksack,
    three: Rucksack,
}

impl RucksackGroup {
    fn parse(input: &Vec<String>) -> Vec<RucksackGroup> {
        let mut groups: Vec<RucksackGroup> = Vec::new();
        let mut stack: Vec<Rucksack> = Vec::new();

        let i = 0;

        for line in input {
            let r = Rucksack::parse(line, i);
            stack.push(r);
        }

        while stack.len() >= 3 {
            let group = RucksackGroup {
                one: stack.pop().unwrap(),
                two: stack.pop().unwrap(),
                three: stack.pop().unwrap(),
            };
            groups.push(group);
        }

        return groups;
    }

    fn badge(&self) -> char {
        let common = common_priorities_three(
            &self.one.elements(),
            &self.two.elements(),
            &self.three.elements(),
        );
        return *common.first().expect("Vector empty");
    }
}

/// Part 1
fn sum_of_priorities(file_name: &str) -> i32 {
    let input = read(file_name).expect("error reading file");
    let rucksacks: Vec<Rucksack> = Rucksack::parse_vec(&input);
    return rucksacks
        .iter()
        .map(|sack| sack.common_priority_sum())
        .sum();
}

/// Part 2
fn sum_of_badges(file_name: &str) -> i32 {
    let input = read(file_name).expect("error reading file");
    let groups: Vec<RucksackGroup> = RucksackGroup::parse(&input);
    return groups.iter().map(|group| group.badge().priority()).sum();
}

#[cfg(test)]
mod tests {
    use super::{common_priorities, sum_of_badges, sum_of_priorities, Priority, Rucksack};

    #[test]
    fn test_priority_a() {
        assert_eq!(1, 'a'.priority())
    }

    #[test]
    fn test_priority_z() {
        assert_eq!(26, 'z'.priority())
    }

    #[test]
    fn test_priority_A() {
        assert_eq!(27, 'A'.priority())
    }
    #[test]
    fn test_priority_Z() {
        assert_eq!(52, 'Z'.priority())
    }

    #[test]
    fn test_parse_rucksack() {
        let s = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        let rucksack = Rucksack {
            index: 1,
            compartmen_one: vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r'],
            compartmen_two: vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p'],
        };
        assert_eq!(rucksack, Rucksack::parse(&s, 1))
    }

    #[test]
    fn test_common_priorities() {
        assert_eq!(
            vec!['p'],
            common_priorities(
                &vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r'],
                &vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p']
            )
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(157, sum_of_priorities("src/day3/test"))
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(7990, sum_of_priorities("src/day3/input"))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(70, sum_of_badges("src/day3/test"))
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(2602, sum_of_badges("src/day3/input"))
    }
}
