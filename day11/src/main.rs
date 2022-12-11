use eval::{Expr, Value};
use num::integer::lcm;
use std::{cell::RefCell, collections::BTreeMap, fs::read_to_string, path::Path};

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: String,
    divisor: u64,
    target: (usize, usize),
    counter: u64,
}

fn part1<P: AsRef<Path>>(path: P) -> u64 {
    process::<20, 3>(path)
}

fn part2<P: AsRef<Path>>(path: P) -> u64 {
    process::<10000, 1>(path)
}

fn process<const N: usize, const K: u64>(path: impl AsRef<Path>) -> u64 {
    let monkeys = get_monkeys(path);
    let lcm = monkeys
        .values()
        .map(RefCell::borrow)
        .map(|m| m.divisor)
        .reduce(lcm)
        .unwrap();
    monkeys
        .values()
        .map(RefCell::borrow_mut)
        .for_each(|mut m| m.operation = format!("({}) % {lcm}", m.operation));
    (0..N).for_each(|_| {
        monkeys.values().map(RefCell::borrow_mut).for_each(|mut m| {
            while let Some(item) = m.items.pop() {
                m.counter += 1;
                let Ok(Value::Number(value)) = Expr::new(&m.operation)
                    .value("old", item)
                    .exec() else {unreachable!()};
                let value = value.as_u64().unwrap() / K;
                let target = if value % m.divisor == 0 {
                    m.target.0
                } else {
                    m.target.1
                };
                monkeys.get(&target).unwrap().borrow_mut().items.push(value);
            }
        });
    });
    let mut business = monkeys
        .into_values()
        .map(RefCell::into_inner)
        .map(|m| m.counter)
        .collect::<Vec<_>>();
    business.sort_unstable();
    business.into_iter().rev().take(2).product()
}

fn get_monkeys<P: AsRef<Path>>(path: P) -> BTreeMap<usize, RefCell<Monkey>> {
    read_to_string(path)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|l| {
            let mut lines = l.split('\n').skip(1);
            RefCell::new(Monkey {
                items: lines.next().unwrap()[18..]
                    .split(", ")
                    .map(|n| n.parse().unwrap())
                    .collect(),
                operation: lines.next().unwrap()[19..].into(),
                divisor: lines.next().unwrap()[21..].parse().unwrap(),
                target: (
                    lines.next().unwrap()[29..].parse().unwrap(),
                    lines.next().unwrap()[30..].parse().unwrap(),
                ),
                counter: 0,
            })
        })
        .enumerate()
        .collect::<BTreeMap<usize, RefCell<Monkey>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 10605);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 2713310158);
    }
}
