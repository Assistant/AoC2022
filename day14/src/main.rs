use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::{BTreeSet, HashMap},
    fs::read_to_string,
    ops::RangeInclusive,
    path::Path,
};

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1<P: AsRef<Path>>(path: P) -> usize {
    process(path, false)
}

fn part2<P: AsRef<Path>>(path: P) -> usize {
    process(path, true)
}

fn range(a: usize, b: usize) -> RangeInclusive<usize> {
    min(a, b)..=max(a, b)
}

fn move_sand(
    sand: &mut (usize, usize),
    new: (usize, usize),
    map: &mut HashMap<usize, BTreeSet<usize>>,
) {
    let old = map.get_mut(&sand.0).unwrap();
    old.remove(&sand.1);
    if old.is_empty() {
        map.remove(&sand.0);
    }
    *sand = (new.0, new.1);
    if let Some(x) = map.get_mut(&new.0) {
        x.insert(new.1);
    } else {
        map.insert(new.0, BTreeSet::from([new.1]));
    }
}

fn process<P: AsRef<Path>>(path: P, floor: bool) -> usize {
    let mut map = get_map(path);
    if floor {
        let height = map
            .values()
            .flat_map(|c| c.iter().max())
            .max()
            .unwrap()
            .clone()
            + 2;
        for x in (500 - (height + 5))..=(505 + height) {
            if let Some(a) = map.get_mut(&x) {
                a.insert(height);
            } else {
                map.insert(x, BTreeSet::from([height]));
            };
        }
    }
    let generator = (500, 0);
    let mut counter = 0;
    'generator: loop {
        let mut sand = generator.clone();
        'mover: loop {
            match map.get(&sand.0).cloned() {
                Some(a) => {
                    let (x, y) = sand;
                    let Some(height) = a.iter().filter(|&&h| h > y).min() else {
                        break 'mover
                    };
                    for x in [x - 1, x + 1] {
                        match { map.get(&x).map(|a| a.contains(height)) } {
                            Some(true) => continue,
                            Some(false) => {
                                move_sand(&mut sand, (x, *height), &mut map);
                                continue 'mover;
                            }
                            None => break 'generator,
                        }
                    }
                    move_sand(&mut sand, (x, *height - 1), &mut map);
                    counter += 1;
                    if sand == generator {
                        break 'generator;
                    }
                    break;
                }
                None => break 'generator,
            }
        }
    }
    counter
}

fn get_map<P: AsRef<Path>>(path: P) -> HashMap<usize, BTreeSet<usize>> {
    let mut map = HashMap::<usize, BTreeSet<usize>>::new();
    read_to_string(path)
        .unwrap()
        .trim()
        .split('\n')
        .map(|l| {
            l.split(" -> ")
                .flat_map(|c| c.split_once(','))
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        })
        .for_each(|r| {
            r.tuple_windows().for_each(|((a_x, a_y), (b_x, b_y))| {
                range(a_x, b_x)
                    .cartesian_product(range(a_y, b_y))
                    .for_each(|(x, y)| {
                        if let Some(a) = map.get_mut(&x) {
                            a.insert(y);
                        } else {
                            map.insert(x, BTreeSet::from([y]));
                        }
                    })
            })
        });
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 24);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 93);
    }
}
