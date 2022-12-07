use itertools::Itertools;

type Signal = Vec<char>;

trait ISignal {
    fn parse(input: &[String]) -> Signal;
    fn find_marker(&self, distinct_chars: i32) -> Option<i32>;
}

impl ISignal for Signal {
    fn parse(input: &[String]) -> Signal {
        input[0].chars().collect()
    }

    fn find_marker(&self, distinct_chars: i32) -> Option<i32> {
        let offset: usize = (distinct_chars - 1).try_into().unwrap();
        if self.len() < offset {
            return None;
        }
        for current in offset..self.len() {
            if self[current - offset..=current].iter().all_unique() {
                return Some(current as i32 + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::{ISignal, Signal};

    fn setup(file_name: &str) -> Signal {
        let input = util::read(file_name);
        Signal::parse(&input)
    }

    #[test]
    fn test_part_1() {
        let signal: Signal = setup("src/day6/test");
        assert_eq!(7, signal.find_marker(4).unwrap())
    }

    #[test]
    fn test_part_1_complete_set() {
        let signal: Signal = setup("src/day6/input");
        assert_eq!(1896, signal.find_marker(4).unwrap())
    }

    #[test]
    fn test_part_2() {
        let signal: Signal = setup("src/day6/test");
        assert_eq!(19, signal.find_marker(14).unwrap())
    }

    #[test]
    fn test_part_2_complete_set() {
        let signal: Signal = setup("src/day6/input");
        assert_eq!(3452, signal.find_marker(14).unwrap())
    }
}
