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
    lines.map(|l| calculate_score(&l.unwrap())).sum()
}

fn calculate_score(play: &str) -> u32 {
    let mut moves = play.split(' ');
    let (Some(oponent), Some(me)) = (moves.next(), moves.next()) else {
        panic!("");
    };
    match (oponent, me) {
        ("A", "X") => 3 + 1,
        ("A", "Y") => 6 + 2,
        ("A", "Z") => 0 + 3,
        ("B", "X") => 0 + 1,
        ("B", "Y") => 3 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "X") => 6 + 1,
        ("C", "Y") => 0 + 2,
        ("C", "Z") => 3 + 3,
        _ => unreachable!(),
    }
}

fn part2<P: AsRef<Path>>(path: P) -> u32 {
    let lines = read_lines(path).unwrap();
    lines.map(|l| calculate_score2(&l.unwrap())).sum()
}

fn calculate_score2(play: &str) -> u32 {
    let mut moves = play.split(' ');
    let (Some(oponent), Some(me)) = (moves.next(), moves.next()) else {
        panic!("");
    };
    match (oponent, me) {
        ("A", "X") => 0 + 3,
        ("A", "Y") => 3 + 1,
        ("A", "Z") => 6 + 2,
        ("B", "X") => 0 + 1,
        ("B", "Y") => 3 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "X") => 0 + 2,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 6 + 1,
        _ => unreachable!(),
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
        assert_eq!(part1(PATH), 15);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 12);
    }
}
