use std::fs::read_to_string;
use std::path::Path;

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1(path: impl AsRef<Path>) -> u32 {
    get_start_index(path, 4)
}

fn part2(path: impl AsRef<Path>) -> u32 {
    get_start_index(path, 14)
}

fn unique(chars: &Vec<char>) -> bool {
    let count = chars.len();
    let mut chars = chars.clone();
    chars.sort();
    chars.dedup();
    chars.len() == count
}

fn get_start_index(path: impl AsRef<Path>, count: usize) -> u32 {
    let input = read_to_string(path).unwrap();
    let mut input: Vec<char> = input.chars().collect();
    let mut count = count;
    let mut seen: Vec<char> = input.drain(0..count).collect();
    while !unique(&seen) {
        seen.drain(0..1);
        seen.push(input.drain(0..1).next().unwrap());
        count += 1;
    }
    count as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 7);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 19);
    }
}
