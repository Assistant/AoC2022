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
    process::<1>(path)
}

fn part2<P: AsRef<Path>>(path: P) -> usize {
    process::<9>(path)
}

fn process<const N: usize>(path: impl AsRef<Path>) -> usize {
    let mut head = (0, 0);
    let mut segments = [(0, 0); N];
    let commands = get_moves(path);
    commands
        .iter()
        .flat_map(|(d, a)| {
            (0..*a)
                .map(|_| {
                    match *d {
                        'R' => head.0 += 1,
                        'L' => head.0 -= 1,
                        'U' => head.1 += 1,
                        'D' => head.1 -= 1,
                        _ => unreachable!(),
                    }
                    segments.iter_mut().fold(head, follow)
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<BTreeSet<(i32, i32)>>()
        .len()
}

fn follow(head: (i32, i32), tail: &mut (i32, i32)) -> (i32, i32) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (a, b) if a.abs() > 1 || b.abs() > 1 => {
            tail.0 += a.signum();
            tail.1 += b.signum();
        }
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
