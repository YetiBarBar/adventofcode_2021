use std::collections::VecDeque;

use adventofcode_tooling::read_single_string_to_t_vec;

pub trait LanternfishGroup {
    fn populate(&mut self, values: &[usize]);
    fn turn(&mut self);
    fn result(&self) -> u128;
}

impl LanternfishGroup for VecDeque<u128> {
    fn populate(&mut self, values: &[usize]) {
        for idx in 0..=8 {
            self.push_back(values.iter().filter(|&&v| v == idx).count() as u128);
        }
    }

    fn turn(&mut self) {
        let first = self.pop_front().unwrap();
        if let Some(v) = self.get_mut(6) {
            *v += first;
        }
        self.push_back(first);
    }

    fn result(&self) -> u128 {
        self.iter().sum()
    }
}

#[must_use]
pub fn process(values: &[usize], turns: usize) -> u128 {
    let mut group = VecDeque::with_capacity(9);
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
pub fn main() {
    let now = std::time::Instant::now();

    let values = read_single_string_to_t_vec("day_2021_6.data", ',');

    println!("Part 1: {}", process(&values, 80));
    println!("Part 2: {}", process(&values, 256));

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
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
