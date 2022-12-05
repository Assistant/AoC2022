#![feature(iter_array_chunks)]
use nom::bytes::complete::tag;
use std::fs::read_to_string;
use std::path::Path;

static FILE: &str = "input.txt";

struct Stacks {
    cols: Vec<Vec<char>>,
}

impl From<String> for Stacks {
    fn from(mut input: String) -> Self {
        let mut stacks: Vec<Vec<char>> = vec![];
        input.push('\n');
        let mut iter = input.split_inclusive('\n').rev();
        for _ in 0..(iter.next().unwrap().len() / 4) {
            stacks.push(vec![])
        }
        iter.for_each(|line| {
            line.chars()
                .array_chunks::<4>()
                .enumerate()
                .for_each(|(i, chars)| {
                    if chars[1].is_ascii_uppercase() {
                        stacks[i].push(chars[1])
                    }
                });
        });

        Self { cols: stacks }
    }
}

impl Stacks {
    fn run(&mut self, command: &str, multi: bool) {
        if command.is_empty() {
            return;
        }
        let (_, (count, from, to)) = Self::parse_command(command).unwrap();
        self.move_crates(count, from, to, multi)
    }

    fn parse_command(input: &str) -> nom::IResult<&str, (usize, usize, usize)> {
        let (input, _) = tag("move ")(input)?;
        let (input, count) = nom::character::complete::u32(input)?;
        let (input, _) = tag(" from ")(input)?;
        let (input, from) = nom::character::complete::u32(input)?;
        let (input, _) = tag(" to ")(input)?;
        let (input, to) = nom::character::complete::u32(input)?;
        Ok((input, (count as usize, from as usize - 1, to as usize - 1)))
    }

    fn move_crates(&mut self, count: usize, from: usize, to: usize, multi: bool) {
        let start = self.cols[from].len() - count;
        let moving: &mut Vec<char> = &mut self.cols[from].drain(start..).collect();
        if !multi {
            moving.reverse();
        }
        self.cols[to].append(moving);
    }

    fn get_top(&self) -> String {
        self.cols.iter().flat_map(|c| c.last()).collect::<String>()
    }
}

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1(path: impl AsRef<Path>) -> String {
    run(path, false)
}

fn part2(path: impl AsRef<Path>) -> String {
    run(path, true)
}

fn run(path: impl AsRef<Path>, multi: bool) -> String {
    let input = read_to_string(path).unwrap();
    let Some((stacks, commands)) = input.split_once("\n\n") else {
        unreachable!()
    };
    let mut stacks = Stacks::from(stacks.to_string());
    for command in commands.split('\n') {
        stacks.run(command, multi);
    }
    stacks.get_top()
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), "CMZ");
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), "MCD");
    }
}
