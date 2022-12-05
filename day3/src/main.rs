#![feature(iter_array_chunks)]
use array_tool::vec::Intersect;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1<P: AsRef<Path>>(path: P) -> u32 {
    let lines = read_lines(path).unwrap();
    lines
        .map(|l| l.unwrap())
        .map(|l| {
            let (first, second) = l.split_at(l.len() / 2);
            let (first, second) = (
                first.chars().collect::<Vec<char>>(),
                second.chars().collect::<Vec<char>>(),
            );
            value(first.intersect(second)[0])
        })
        .sum()
}

fn part2<P: AsRef<Path>>(path: P) -> u32 {
    let lines = read_lines(path).unwrap();
    lines
        .map(|l| l.unwrap())
        .array_chunks::<3>()
        .map(|c| {
            let [one, two, three] = c.map(|i| i.chars().collect::<Vec<char>>());
            value(one.intersect(two.intersect(three))[0])
        })
        .sum()
}

fn value(character: char) -> u32 {
    let value = character as u32;
    match value {
        0..=90 => value - 38,
        _ => value - 96,
    }
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
        assert_eq!(part1(PATH), 157);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 70);
    }
}
