use std::fs::read_to_string;
use std::path::Path;

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1<P: AsRef<Path>>(path: P) -> u32 {
    let mut elves = list_elves(path);

    elves.pop().unwrap()
}

fn part2<P: AsRef<Path>>(path: P) -> u32 {
    let mut elves = list_elves(path);

    elves.reverse();
    elves[0..3].iter().sum()
}

fn list_elves<P: AsRef<Path>>(path: P) -> Vec<u32> {
    let mut elves: Vec<u32> = read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|e| e.split('\n').flat_map(|s| s.parse::<u32>()).sum())
        .collect();

    elves.sort();
    elves
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 24000);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 45000);
    }
}
