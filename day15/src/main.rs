use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::i32, IResult};
use std::{collections::HashMap, fs::read_to_string, path::Path};

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE, 2_000_000);
    println!("{one}");

    let two = part2(FILE, 4_000_000);
    println!("{two}");
}

fn part1<P: AsRef<Path>>(path: P, line: i32) -> usize {
    let sensors = get_sensors(path);
    sensors
        .iter()
        .filter_map(|((s_x, s_y), &(b_x, b_y))| {
            let distance = (s_x - b_x).abs() + (s_y - b_y).abs();
            if ((s_y - distance)..=(s_y + distance)).contains(&line) {
                let radius = distance - (s_y - line).abs();
                Some((s_x - radius)..=(s_x + radius))
            } else {
                None
            }
        })
        .flatten()
        .filter(|&x| !sensors.values().contains(&(x, line)))
        .unique()
        .count()
}

fn part2<P: AsRef<Path>>(path: P, max: i32) -> u64 {
    let sensors = get_sensors(path)
        .into_iter()
        .map(|((x, y), (b_x, b_y))| ((x, y), (x - b_x).abs() + (y - b_y).abs()))
        .collect::<Vec<_>>();

    let (x, y) = inside(0, 0, &sensors, max, max)[0];
    x as u64 * 4_000_000 + y as u64
}

fn inside(x: i32, y: i32, map: &Vec<((i32, i32), i32)>, x_max: i32, y_max: i32) -> Vec<(i32, i32)> {
    const DIV: usize = 100;
    let size = DIV as i32;

    if (x_max - x) <= size && (y_max - y) < size {
        (x..=x_max)
            .cartesian_product(y..=y_max)
            .filter(|(x, y)| {
                map.iter()
                    .all(|&((s_x, s_y), distance)| (x - s_x).abs() + (y - s_y).abs() > distance)
            })
            .collect()
    } else {
        let x_iter = (x..=(x + x_max)).chunks(DIV);
        let y_iter = (y..=(y + y_max)).chunks(DIV);
        let x_range = x_iter
            .into_iter()
            .map(Iterator::collect::<Vec<_>>)
            .collect::<Vec<_>>();
        let y_range = y_iter
            .into_iter()
            .map(Iterator::collect::<Vec<_>>)
            .collect::<Vec<_>>();

        x_range
            .iter()
            .cartesian_product(y_range.iter())
            .map(|(p_x, p_y)| {
                (
                    (p_x.first().unwrap(), p_y.first().unwrap()),
                    (p_x.last().unwrap(), p_y.last().unwrap()),
                )
            })
            .filter(|&((&x, &y), (&x_max, &y_max))| {
                !map.iter().any(|&((s_x, s_y), distance)| {
                    [(x, y), (x_max, y), (x, y_max), (x_max, y_max)]
                        .iter()
                        .all(|(x, y)| (x - s_x).abs() + (y - s_y).abs() < distance)
                })
            })
            .flat_map(|((&x, &y), (&x_max, &y_max))| inside(x, y, map, x_max, y_max))
            .collect()
    }
}

fn get_sensors<P: AsRef<Path>>(path: P) -> HashMap<(i32, i32), (i32, i32)> {
    read_to_string(path)
        .unwrap()
        .trim()
        .split('\n')
        .flat_map(sensor)
        .map(|(_, a)| a)
        .collect()
}

fn sensor(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = i32(input)?;
    Ok((input, ((sensor_x, sensor_y), (beacon_x, beacon_y))))
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH, 10), 26);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH, 20), 56000011);
    }
}
