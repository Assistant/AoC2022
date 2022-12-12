use petgraph::{algo::astar, Graph};
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
    process(false, -14, vec![-28], path)
}

fn part2<P: AsRef<Path>>(path: P) -> usize {
    process(true, -28, vec![-14, 0], path)
}

fn process(reverse: bool, start: i8, end: Vec<i8>, path: impl AsRef<Path>) -> usize {
    let graph = get_map(path, reverse);
    let mut indieces = graph.node_indices();
    let start = indieces
        .find(|p| *graph.node_weight(*p).unwrap() == start)
        .unwrap();
    let path = astar(
        &graph,
        start,
        |e| end.contains(graph.node_weight(e).unwrap()),
        |_| 1,
        |_| 0,
    );
    let Some((ans, _)) = path else { unreachable!() };
    ans
}

fn get_map(path: impl AsRef<Path>, reverse: bool) -> Graph<i8, ()> {
    let mut graph = Graph::new();

    let map = read_to_string(path).unwrap();
    let map = map
        .trim()
        .split('\n')
        .map(|l| {
            l.chars()
                .map(|c| graph.add_node(c as i8 - 'a' as i8))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    for (y, row) in map.iter().enumerate() {
        for (x, &node) in row.iter().enumerate() {
            let value = *graph.node_weight(node).unwrap();
            let (x, y) = (x as i32, y as i32);
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .filter_map(|&(x, y)| map.get(y as usize).and_then(|m| m.get(x as usize)))
                .for_each(|&n| {
                    let value = normialize(value);
                    let &neighbors_value = graph.node_weight(n).unwrap();
                    let neighbors_value = normialize(neighbors_value);

                    let edge_test = if reverse {
                        neighbors_value - value
                    } else {
                        value - neighbors_value
                    };
                    if edge_test >= -1 {
                        graph.add_edge(node, n, ());
                    }
                });
        }
    }
    graph
}

fn normialize(value: i8) -> i8 {
    match value {
        -14 => 0,
        -28 => 25,
        _ => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 31);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 29);
    }
}
