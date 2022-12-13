use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list0, sequence::delimited, IResult,
};
use std::{cmp::Ordering, fs::read_to_string, path::Path};

static FILE: &str = "input.txt";

#[derive(Clone, Debug, Ord, Eq, PartialEq)]
enum Signal {
    List(Vec<Signal>),
    Item(u8),
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Item(a), Self::Item(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => {
                let list = a.iter().zip(b).flat_map(|(a, b)| a.partial_cmp(b));
                for value in list {
                    if [Ordering::Less, Ordering::Greater].contains(&value) {
                        return Some(value);
                    }
                }
                a.len().partial_cmp(&b.len())
            }
            (Self::List(a), b) => Self::List(a.clone()).partial_cmp(&Self::List(vec![b.clone()])),
            (a, b) => (*b).partial_cmp(a).map(Ordering::reverse),
        }
    }
}

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1<P: AsRef<Path>>(path: P) -> i32 {
    get_signals(path)
        .iter()
        .zip(1..)
        .map(|([a, b], i)| if a < b { i } else { 0 })
        .sum()
}

fn part2<P: AsRef<Path>>(path: P) -> i32 {
    let dividers = [2, 6]
        .iter()
        .map(|i| Signal::List(vec![Signal::List(vec![Signal::Item(*i)])]))
        .collect::<Vec<_>>();
    let mut signals = get_signals(path).into_iter().flatten().collect::<Vec<_>>();
    signals.append(&mut dividers.clone());
    signals.sort();
    signals
        .iter()
        .zip(1..)
        .filter(|(s, _)| dividers.contains(s))
        .map(|(_, i)| i)
        .product()
}

fn signal(input: &str) -> Signal {
    let (_, result) = list(input).unwrap();
    result
}

fn list(input: &str) -> IResult<&str, Signal> {
    delimited(
        tag("["),
        separated_list0(tag(","), alt((list, item))),
        tag("]"),
    )(input)
    .map(|(i, s)| (i, Signal::List(s)))
}

fn item(input: &str) -> IResult<&str, Signal> {
    map_res(digit1, str::parse)(input).map(|(i, v)| (i, Signal::Item(v)))
}

fn get_signals<P: AsRef<Path>>(path: P) -> Vec<[Signal; 2]> {
    read_to_string(path)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|l| {
            let line = l.split_once('\n').unwrap();
            [signal(line.0), signal(line.1)]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 13);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 140);
    }
}
