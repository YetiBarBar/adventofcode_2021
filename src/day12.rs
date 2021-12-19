use std::{fs::read_to_string, path::PathBuf};

use adventofcode_tooling::AocError;
use hashbrown::{HashMap, HashSet};

#[must_use]
fn part_1(data: &HashMap<String, HashSet<String>>) -> usize {
    let mut visited_small_caves = [String::from("start")]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    recursive_traversal(data, "start", &mut visited_small_caves)
}

fn recursive_traversal(
    data: &HashMap<String, HashSet<String>>,
    current_cave: &str,
    visited_small_caves: &mut HashSet<String>,
) -> usize {
    if current_cave == "end" {
        visited_small_caves.remove("end");
        return 1;
    }

    let mut res = 0;

    if let Some(connected_caves) = data.get(current_cave) {
        for connected_cave in connected_caves {
            if is_small_cave(connected_cave) {
                if visited_small_caves.contains(connected_cave) {
                    continue;
                }
                visited_small_caves.insert(connected_cave.to_string());
            }
            res += recursive_traversal(data, connected_cave, visited_small_caves);
            visited_small_caves.remove(connected_cave);
        }
    }
    res
}

fn recursive_traversal_complex(
    data: &HashMap<String, HashSet<String>>,
    current_cave: &str,
    visited_small_caves: &mut HashSet<String>,
    twice_visited: &mut Option<String>,
) -> usize {
    if current_cave == "end" {
        visited_small_caves.remove("end");
        return 1;
    }

    let mut res = 0;

    if let Some(connected_caves) = data.get(current_cave) {
        for connected_cave in connected_caves {
            if is_small_cave(connected_cave) {
                if !visited_small_caves.contains(connected_cave) {
                    visited_small_caves.insert(connected_cave.to_string());
                } else if twice_visited.is_none()
                    && connected_cave != "start"
                    && connected_cave != "end"
                {
                    *twice_visited = Some(connected_cave.to_string());
                } else {
                    continue;
                }
            }

            if let Some(val) = twice_visited {
                res += recursive_traversal(data, connected_cave, visited_small_caves);
                if val == connected_cave {
                    *twice_visited = None;
                }
            } else {
                res += recursive_traversal_complex(
                    data,
                    connected_cave,
                    visited_small_caves,
                    twice_visited,
                );
                visited_small_caves.remove(connected_cave);
            }
        }
    }
    res
}

#[must_use]
fn part_2(data: &HashMap<String, HashSet<String>>) -> usize {
    let mut visited_small_caves = [(String::from("start"))]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

    let mut twice_visited = None;
    recursive_traversal_complex(data, "start", &mut visited_small_caves, &mut twice_visited)
}

#[must_use]
pub fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_lowercase())
}

fn parse_input<T: AsRef<str>>(data: T) -> HashMap<String, HashSet<String>> {
    let mut hmap = HashMap::new();
    for line in data.as_ref().lines() {
        let parts: Vec<String> = line.split('-').map(|s| s.trim().to_string()).collect();
        let a = parts[0].clone();
        let b = parts[1].clone();
        hmap.entry(a.clone())
            .or_insert_with(HashSet::new)
            .insert(b.clone());
        hmap.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    hmap
}

/// Process solutions for day 12
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_12.data");
    let input_data = read_to_string(filepath).unwrap();

    let data = parse_input(&input_data);
    println!("Part 1: {:?}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day12_part_1() {
        let input_data = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

        let input_data = parse_input(&input_data);
        assert_eq!(part_1(&input_data), 10);
    }

    #[test]
    fn test_day12_part_2() {
        let input_data = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

        let input_data = parse_input(&input_data);
        assert_eq!(part_2(&input_data), 36);
    }
}
