type Tree = usize;

type Forest = Vec<Vec<Tree>>;

trait IForest {
    fn search(&self) -> Vec<Tree>;
    fn is_visible(&self, a: &usize, b: &usize) -> bool;
    fn parse(input: &[String]) -> Forest;

    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn south(&self, x: &usize, y: &usize) -> bool;
    fn north(&self, x: &usize, y: &usize) -> bool;
    fn east(&self, x: &usize, y: &usize) -> bool;
    fn west(&self, x: &usize, y: &usize) -> bool;
    fn edge(&self, x: &usize, y: &usize) -> bool;
    fn row(s: &str) -> Vec<Tree>;
}

impl IForest for Forest {
    fn search(&self) -> Vec<Tree> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .filter(|(x, y)| self.is_visible(x, y))
            .map(|(x, y)| self[y][x])
            .collect()
    }

    fn is_visible(&self, x: &usize, y: &usize) -> bool {
        self.edge(x, y)
            || self.north(x, y)
            || self.south(x, y)
            || self.east(x, y)
            || self.west(x, y)
    }

    fn parse(input: &[String]) -> Forest {
        input.iter().map(|string| Forest::row(string)).collect()
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn width(&self) -> usize {
        self[0].len()
    }

    fn south(&self, x: &usize, y: &usize) -> bool {
        self[y + 1..self.height()]
            .iter()
            .map(|v| &v[*x])
            .all(|t| &self[*y][*x] > t)
    }

    fn north(&self, x: &usize, y: &usize) -> bool {
        self[0..*y]
            .iter()
            .map(|v| &v[*x])
            .all(|t| &self[*y][*x] > t)
    }

    fn east(&self, x: &usize, y: &usize) -> bool {
        self[*y][x + 1..self.width()]
            .iter()
            .all(|t| &self[*y][*x] > t)
    }

    fn west(&self, x: &usize, y: &usize) -> bool {
        self[*y][0..*x].iter().all(|t| &self[*y][*x] > t)
    }

    fn edge(&self, x: &usize, y: &usize) -> bool {
        *x == 0 || *y == 0 || *y == self.height() - 1 || *x == self.width() - 1
    }

    fn row(string: &str) -> Vec<Tree> {
        return string
            .chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::{Forest, IForest};
    use crate::util;

    fn setup(file_name: &str) -> Forest {
        let input = util::read(file_name);
        Forest::parse(&input)
    }

    #[test]
    fn test_part_1() {
        let forest = setup("src/day8/test");
        assert_eq!(
            vec![3, 0, 3, 7, 3, 2, 5, 5, 2, 6, 5, 3, 2, 3, 5, 9, 3, 5, 3, 9, 0],
            forest.search()
        );
    }

    #[test]
    fn test_part_1_complete() {
        let forest = setup("src/day8/input");
        assert_eq!(1870, forest.search().len());
    }
}
