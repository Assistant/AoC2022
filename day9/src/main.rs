use std::collections::BTreeSet;
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
    let mut tail = (0, 0);
    process(path, &mut |h| follow(h, &mut tail))
}

fn part2<P: AsRef<Path>>(path: P) -> usize {
    let mut tails = [(0, 0); 9];
    process(path, &mut |h| tails.iter_mut().fold(h, follow))
}

fn process<P: AsRef<Path>, F: FnMut((i32, i32)) -> (i32, i32)>(path: P, func: &mut F) -> usize {
    let mut head = (0, 0);
    let commands = get_moves(path);
    commands
        .iter()
        .flat_map(|(d, a)| {
            (0..*a)
                .map(|_| {
                    match *d {
                        'R' => head = (head.0 + 1, head.1),
                        'L' => head = (head.0 - 1, head.1),
                        'U' => head = (head.0, head.1 + 1),
                        'D' => head = (head.0, head.1 - 1),
                        _ => unreachable!(),
                    }
                    func(head)
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<BTreeSet<(i32, i32)>>()
        .len()
}

fn follow(head: (i32, i32), tail: &mut (i32, i32)) -> (i32, i32) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (a, b) if a > 1 && b == 0 => *tail = (tail.0 + 1, tail.1),
        (a, b) if a == 0 && b > 1 => *tail = (tail.0, tail.1 + 1),
        (a, b) if a < -1 && b == 0 => *tail = (tail.0 - 1, tail.1),
        (a, b) if a == 0 && b < -1 => *tail = (tail.0, tail.1 - 1),
        (a, b) if a + b > 2 => *tail = (tail.0 + 1, tail.1 + 1),
        (a, b) if a + b.abs() > 2 => *tail = (tail.0 + 1, tail.1 - 1),
        (a, b) if a.abs() + b > 2 => *tail = (tail.0 - 1, tail.1 + 1),
        (a, b) if a + b < -2 => *tail = (tail.0 - 1, tail.1 - 1),
        _ => {}
    }
    *tail
}

fn get_moves<P: AsRef<Path>>(path: P) -> Vec<(char, usize)> {
    let input = read_to_string(path).unwrap();
    input
        .trim()
        .split('\n')
        .map(|l| {
            let (c, a) = l.split_once(' ').unwrap();
            (c.chars().next().unwrap(), a.parse().unwrap())
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
        assert_eq!(part2(PATH), 1);
    }
}
