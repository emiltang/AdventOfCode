use std::ops::Range;

#[derive(Debug, PartialEq)]
struct Elf(Range<i32>);

impl Elf {
    fn overlap_partial(a: &Elf, b: &Elf) -> bool {
        matches!((a, b), (Elf(x), Elf(y)) if x.end >= y.start && x.start <= y.end)
    }

    fn overlap_complete<'a>(a: &'a Elf, b: &'a Elf) -> Option<&'a Elf> {
        match (a, b) {
            (Elf(e), Elf(f)) if e.start <= f.start && e.end >= f.end => Some(a),
            (Elf(e), Elf(f)) if e.start >= f.start && e.end <= f.end => Some(b),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Pair {
    first: Elf,
    second: Elf,
}

impl Pair {
    fn parse(s: &str) -> Pair {
        let x: Vec<&str> = s.split(&[',', '-']).collect();

        let a: i32 = x[0].parse().unwrap();
        let b: i32 = x[1].parse().unwrap();
        let c: i32 = x[2].parse().unwrap();
        let d: i32 = x[3].parse().unwrap();

        Pair {
            first: Elf(a..b),
            second: Elf(c..d),
        }
    }

    fn parse_vec(input: &[String]) -> Vec<Pair> {
        input.iter().map(|s| Pair::parse(s)).collect()
    }

    fn is_overlapping(&self) -> bool {
        return Elf::overlap_complete(&self.first, &self.second).is_some();
    }

    fn overlapping_pairs(pairs: &[Pair]) -> Vec<&Pair> {
        pairs.iter().filter(|p| p.is_overlapping()).collect()
    }

    fn partial_overlapping_pairs(pairs: &[Pair]) -> Vec<&Pair> {
        return pairs
            .iter()
            .filter(|p| Elf::overlap_partial(&p.first, &p.second))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::{Elf, Pair};

    #[test]
    fn test_parse_pair() {
        let pair = Pair {
            first: Elf(2..4),
            second: Elf(6..8),
        };
        let string = String::from("2-4,6-8");
        assert_eq!(pair, Pair::parse(&string));
    }

    #[test]
    fn test_contained() {
        let a = Elf(2..8);
        let b = Elf(3..7);
        assert_eq!(Some(&a), Elf::overlap_complete(&a, &b))
    }

    #[test]
    fn test_contained_2() {
        let a = Elf(2..8);
        let b = Elf(3..9);
        assert_eq!(None, Elf::overlap_complete(&a, &b))
    }

    #[test]
    fn test_contained_3() {
        let a = Elf(2..8);
        let b = Elf(1..7);
        assert_eq!(None, Elf::overlap_complete(&a, &b))
    }

    #[test]
    fn test_contained_4() {
        let a = Elf(2..8);
        let b = Elf(1..9);
        assert_eq!(Some(&b), Elf::overlap_complete(&a, &b))
    }

    #[test]
    fn test_contained_5() {
        let a = Elf(2..8);
        let b = Elf(3..7);
        assert!(Elf::overlap_partial(&a, &b))
    }

    #[test]
    fn test_contained_6() {
        let a = Elf(2..8);
        let b = Elf(1..9);
        assert!(Elf::overlap_partial(&a, &b))
    }

    #[test]
    fn test_contained_7() {
        let a = Elf(2..8);
        let b = Elf(3..9);
        assert!(Elf::overlap_partial(&a, &b))
    }

    #[test]
    fn test_contained_8() {
        let a = Elf(2..8);
        let b = Elf(1..7);
        assert!(Elf::overlap_partial(&a, &b))
    }

    #[test]
    fn test_contained_9() {
        let a = Elf(2..5);
        let b = Elf(6..7);
        assert!(!Elf::overlap_partial(&a, &b))
    }

    #[test]
    fn test_contained_10() {
        let a = Elf(8..10);
        let b = Elf(1..4);
        assert!(!Elf::overlap_partial(&a, &b))
    }

    #[test]
    fn test_part_1() {
        let lines = util::read("src/day4/test");
        let pairs = Pair::parse_vec(&lines);
        let overlapping_pairs = Pair::overlapping_pairs(&pairs);
        assert_eq!(2, overlapping_pairs.len())
    }

    #[test]
    fn test_part_1_complete_set() {
        let lines = util::read("src/day4/input");
        let pairs = Pair::parse_vec(&lines);
        let overlapping_pairs = Pair::overlapping_pairs(&pairs);
        assert_eq!(500, overlapping_pairs.len())
    }

    #[test]
    fn test_part_2() {
        let lines = util::read("src/day4/test");
        let pairs = Pair::parse_vec(&lines);
        let overlapping_pairs = Pair::partial_overlapping_pairs(&pairs);
        assert_eq!(4, overlapping_pairs.len())
    }

    #[test]
    fn test_part_2_complete_set() {
        let lines = util::read("src/day4/input");
        let pairs = Pair::parse_vec(&lines);
        let overlapping_pairs = Pair::partial_overlapping_pairs(&pairs);
        assert_eq!(815, overlapping_pairs.len())
    }
}
