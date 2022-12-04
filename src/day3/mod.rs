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

    fn calc_priority(&self) -> i32 {
        let common_priorities = common_priorities(&self.compartmen_one, &self.compartmen_two);
        return common_priorities.iter().map(|c| c.priority()).sum();
    }
}

fn common_priorities(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut vec: Vec<char> = a.iter().filter(|c| b.contains(c)).map(|c| *c).collect();
    vec.sort();
    vec.dedup();
    return vec;
}

fn sum_of_priorities(file_name: &str) -> i32 {
    let input = read(file_name).expect("error reading file");
    let rucksacks: Vec<Rucksack> = Rucksack::parse_vec(&input);
    return rucksacks.iter().map(|r| r.calc_priority()).sum();
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

#[cfg(test)]
mod tests {
    use super::{common_priorities, sum_of_priorities, Priority, Rucksack};

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
}
