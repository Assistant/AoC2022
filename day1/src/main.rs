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
    let mut elves = list_elves(path);

    elves.pop().unwrap()
}

fn part2<P: AsRef<Path>>(path: P) -> u32 {
    let mut elves = list_elves(path);

    elves.reverse();
    elves[0..3].iter().sum()
}

fn list_elves<P: AsRef<Path>>(path: P) -> Vec<u32> {
    let mut elves = vec![];
    let mut total_calories = 0;

    let lines = read_lines(path).unwrap();
    for line in lines {
        let line = line.unwrap();
        match line {
            _ if line.is_empty() => {
                elves.push(total_calories);
                total_calories = 0;
            }
            calories => {
                total_calories += calories.parse::<u32>().unwrap();
            }
        }
    }
    elves.sort();
    elves
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
        assert_eq!(part1(PATH), 24000);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 45000);
    }
}
