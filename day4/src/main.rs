use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::path::Path;

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1(path: impl AsRef<Path>) -> u32 {
    process(path, |a, b| {
        (
            a.clone().all(|i| b.contains(&i)),
            b.clone().all(|i| a.contains(&i)),
        )
    })
}

fn part2(path: impl AsRef<Path>) -> u32 {
    process(path, |a, b| {
        (
            a.clone().any(|i| b.contains(&i)),
            b.clone().any(|i| a.contains(&i)),
        )
    })
}

fn process(
    path: impl AsRef<Path>,
    func: impl Fn(RangeInclusive<i32>, RangeInclusive<i32>) -> (bool, bool),
) -> u32 {
    let lines = read_lines(path).unwrap();
    lines
        .map(|l| l.unwrap())
        .map(|l| {
            let Some((a,b)) = l.split_once(',') else {
                unreachable!();
            };
            let (a, b) = (get_range(a), get_range(b));
            match func(a, b) {
                (true, _) | (_, true) => 1,
                _ => 0,
            }
        })
        .sum()
}

fn get_range(range: &str) -> RangeInclusive<i32> {
    let range = range.split_once('-').unwrap();
    range.0.parse().unwrap()..=range.1.parse().unwrap()
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 2);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 4);
    }
}
