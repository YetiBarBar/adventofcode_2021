use std::num::NonZeroUsize;

use adventofcode_tooling::read_lines_to_vec_t;

/// Process data for a given step
///
/// # Errors
///
/// step has to be greater than 1
#[must_use]
pub fn process(data: &[usize], step: NonZeroUsize) -> usize {
    let step: usize = step.into();
    data.windows(step)
        .filter(|&windows| windows[step.saturating_sub(1)] > windows[0])
        .count()
}

/// Process data for a given step
#[must_use]
pub fn part_1(data: &[usize]) -> usize {
    process(data, unsafe { NonZeroUsize::new_unchecked(2) })
}

/// Process data for a given step
#[must_use]
pub fn part_2(data: &[usize]) -> usize {
    process(data, unsafe { NonZeroUsize::new_unchecked(4) })
}

/// Process solutions for day 1
pub fn main() {
    let now = std::time::Instant::now();

    let values = read_lines_to_vec_t("day_2021_1.data");
    println!("Part 1: {}", part_1(&values));
    println!("Part 2: {}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_step1() {
        let values: &[usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part_1(values), 7);
    }

    #[test]
    fn test_day1_step2() {
        let values: &[usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part_2(values), 5);
    }
}
