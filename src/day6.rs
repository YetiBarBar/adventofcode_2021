use std::{collections::VecDeque, path::PathBuf};

struct LanternfishGroup {
    group: VecDeque<u128>,
}

impl LanternfishGroup {
    pub fn new() -> Self {
        Self {
            group: VecDeque::<u128>::with_capacity(8),
        }
    }
    pub fn populate(&mut self, values: &[usize]) {
        for idx in 0..=8 {
            self.group
                .push_back(values.iter().filter(|&&v| v == idx).count() as u128);
        }
    }

    pub fn turn(&mut self) {
        let first = self.group.pop_front().unwrap();
        if let Some(v) = self.group.get_mut(6) {
            *v += first;
        }
        self.group.push_back(first);
    }

    pub fn result(&self) -> u128 {
        self.group.iter().sum()
    }
}

#[must_use]
pub fn process(values: &[usize], turns: usize) -> u128 {
    let mut group = LanternfishGroup::new();
    group.populate(values);
    for _ in 0..turns {
        group.turn();
    }
    group.result()
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_6.data");

    let input_data = std::fs::read_to_string(filepath)?;
    let values = input_data
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", process(&values, 80));
    println!("Part 2: {:?}", process(&values, 256));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day6_part1() {
        let values = [3_usize, 4, 3, 1, 2];

        assert_eq!(process(&values, 80), 5934);
    }
    #[test]
    fn test_day6_part2() {
        let values = [3_usize, 4, 3, 1, 2];

        assert_eq!(process(&values, 256), 26984457539);
    }
}
