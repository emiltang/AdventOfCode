mod map;

#[derive(Debug)]
struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    fn new(x: i32, y: i32, x_max: i32, x_min: i32, y_max: i32, y_min: i32) -> Pair {
        Pair {
            x: norm(x, x_max, x_min),
            y: norm(y, y_max, y_min),
        }
    }
}

type Path = Vec<Pair>;

fn parse(input: &[String]) -> Vec<Path> {
    input
        .iter()
        .map(|line| {
            line.split("->")
                .map(|s| s.trim())
                .map(|s| {
                    let (xs, ys) = s.split_once(',').unwrap();
                    let x: i32 = xs.parse().unwrap();
                    let y: i32 = ys.parse().unwrap();
                    Pair { x, y }
                })
                .collect()
        })
        .collect()
}

fn flat_max(paths: &[Path], fun: fn(&Pair) -> i32) -> i32 {
    paths.iter().flat_map(|v| v.iter().map(fun)).max().unwrap()
}

fn flat_min(paths: &[Path], fun: fn(&Pair) -> i32) -> i32 {
    paths.iter().flat_map(|v| v.iter().map(fun)).min().unwrap()
}

fn norm(k: i32, max: i32, min: i32) -> i32 {
    (k - min) / (max - min)
}


fn normalize(paths: &[Path]) -> Vec<Path> {
    let x_max = flat_max(paths, |p| p.x);
    let y_max = flat_max(paths, |p| p.y);
    let x_min = flat_min(paths, |p| p.x);
    let y_min = flat_min(paths, |p| p.y);

    paths
        .iter()
        .map(|v| {
            v.iter()
                .map(|p| Pair::new(p.x, p.y, x_max, x_min, y_max, y_min))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::parse;

    #[test]
    fn test() {
        let input = util::read("src/day14/test");
        assert_eq!(2, parse(&input).len());
    }
}
