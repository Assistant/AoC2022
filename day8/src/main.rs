#![feature(array_zip)]

use std::cmp::min;
use std::fs::read_to_string;
use std::path::Path;

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1<P: AsRef<Path>>(path: P) -> usize {
    let (map, width, height) = get_matix(path);
    map.iter()
        .enumerate()
        .map(|(row, r)| {
            if row == 0 || row == height {
                return r.len();
            }
            r.iter()
                .enumerate()
                .map(|(col, c)| {
                    (col == 0
                        || col == width
                        || r[..col].iter().all(|i| c > i)
                        || r[col + 1..].iter().all(|i| c > i)
                        || map[..row].iter().all(|i| *c > i[col])
                        || map[row + 1..].iter().all(|i| *c > i[col])) as usize
                })
                .sum()
        })
        .sum()
}

fn part2<P: AsRef<Path>>(path: P) -> usize {
    let (map, width, height) = get_matix(path);
    map.iter()
        .enumerate()
        .flat_map(|(row, r)| {
            r.iter()
                .enumerate()
                .flat_map(|(col, c)| {
                    [
                        r[..col].iter().rev().take_while(|&i| c > i).count(),
                        r[col + 1..].iter().take_while(|&i| c > i).count(),
                        map[..row].iter().rev().take_while(|&i| *c > i[col]).count(),
                        map[row + 1..].iter().take_while(|&i| *c > i[col]).count(),
                    ]
                    .zip([col > 0, col < width, row > 0, row < height])
                    .map(|(a, b)| a + b as usize)
                    .zip([col, width - col, row, height - row])
                    .map(|(a, b)| min(a, b))
                    .into_iter()
                    .reduce(|a, n| a * n)
                })
                .max()
        })
        .max()
        .unwrap()
}

fn get_matix<P: AsRef<Path>>(path: P) -> (Vec<Vec<char>>, usize, usize) {
    let input = read_to_string(path).unwrap();
    let input = input.trim().split('\n').collect::<Vec<&str>>();
    let height = input.len() - 1;
    let width = input[0].len() - 1;

    let map = input
        .iter()
        .map(|&l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    (map, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 21);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 8);
    }
}
