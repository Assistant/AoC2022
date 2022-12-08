use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

static FILE: &str = "input.txt";

fn main() {
    let one = part1(FILE);
    println!("{one}");

    let two = part2(FILE);
    println!("{two}");
}

fn part1(path: impl AsRef<Path>) -> u32 {
    let dirs = get_dirs(path);
    dirs.iter()
        .map(|(name, c)| calculate_size(c, &dirs, name))
        .filter(|s| s <= &100_000)
        .sum()
}

fn part2(path: impl AsRef<Path>) -> u32 {
    let dirs = get_dirs(path);
    let required_space = 30_000_000 - (70_000_000 - calculate_size(&dirs["/"], &dirs, "/"));
    dirs.iter()
        .map(|(name, c)| calculate_size(c, &dirs, name))
        .filter(|s| s >= &required_space)
        .min()
        .unwrap()
}

fn get_dirs(path: impl AsRef<Path>) -> HashMap<String, Vec<String>> {
    let input = read_to_string(path).unwrap();
    let mut dirs: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_dir = String::new();
    input
        .split('$')
        .skip(2)
        .map(str::trim)
        .for_each(|c| match c {
            "cd .." => current_dir = current_dir[..current_dir.rfind('/').unwrap()].into(),
            dir if dir.starts_with("cd ") => current_dir = format!("{current_dir}/{}", &dir[3..]),
            files if files.starts_with("ls\n") => {
                let dir = if current_dir.is_empty() {
                    "/".into()
                } else {
                    current_dir.clone()
                };
                dirs.insert(dir, (&files[3..]).split('\n').map(str::to_string).collect());
            }
            _ => unreachable!(),
        });
    dirs
}

fn calculate_size(dir: &Vec<String>, tree: &HashMap<String, Vec<String>>, path: &str) -> u32 {
    dir.iter()
        .flat_map(|d| d.split_once(' '))
        .map(|v| match v {
            ("dir", dir) => {
                let path = if path == "/" {
                    format!("/{dir}")
                } else {
                    format!("{path}/{dir}")
                };
                calculate_size(tree.get(&path).unwrap(), tree, &path)
            }
            (size, _) => size.parse().unwrap(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static PATH: &str = "test.txt";

    #[test]
    fn one() {
        assert_eq!(part1(PATH), 95437);
    }

    #[test]
    fn two() {
        assert_eq!(part2(PATH), 24933642);
    }
}
