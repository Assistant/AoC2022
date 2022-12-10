#![feature(iter_array_chunks, iter_intersperse)]

use std::fs::read_to_string;
use std::path::Path;

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1<P: AsRef<Path>>(path: P) -> i32 {
    let values = [20, 60, 100, 140, 180, 220];
    register_values(path)
        .iter()
        .zip(1..)
        .filter(|(_, i)| values.contains(i))
        .map(|(v, i)| v * i)
        .sum()
}

fn part2<P: AsRef<Path>>(path: P) -> String {
    register_values(path)
        .into_iter()
        .zip(0..)
        .map(|(v, i)| if (i % 40 - v).abs() < 2 { '#' } else { '.' })
        .array_chunks::<40>()
        .map(|c| c.iter().collect::<String>())
        .intersperse('\n'.into())
        .collect()
}

fn register_values<P: AsRef<Path>>(path: P) -> Vec<i32> {
    let mut register = 1;
    [1].into_iter()
        .chain(
            read_to_string(path)
                .unwrap()
                .trim()
                .split('\n')
                .map(|c| match c {
                    "noop" => (0, 0),
                    add => (1, add[5..].parse().unwrap()),
                })
                .flat_map(|(d, v)| {
                    let start = register;
                    register += v;
                    [vec![start; d], vec![register]]
                        .into_iter()
                        .collect::<Vec<Vec<_>>>()
                })
                .flatten(),
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";
    static IMG: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 13140);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), IMG);
    }
}
