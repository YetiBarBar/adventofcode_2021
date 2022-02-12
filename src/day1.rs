use std::num::NonZeroUsize;

use adventofcode_tooling::{read_lines_to_vec_t, AocError};

/// Process data for a given step
///
/// # Errors
///
/// step has to be greater than 1
pub fn process(data: &[usize], step: NonZeroUsize) -> usize {
    // if data.len().lt(&step) {
    //     return Err("Trying to process a step larger than data");
    // }
    // match step {
    //     0 => Err("Invalid step: 0"),
    //     _ => Ok(data
    //         .windows(step)
    //         .filter(|&window| window[step - 1] > window[0])
    //         .count()),
    // }
    let step: usize = step.into();
    data.windows(step)
        .filter(|&windows| windows[step.saturating_sub(1)] > windows[0])
        .count()
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1(data: &[usize]) -> usize {
    process(data, NonZeroUsize::new(2).unwrap())
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2(data: &[usize]) -> usize {
    process(data, NonZeroUsize::new(4).unwrap())
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    let values = read_lines_to_vec_t("day_2021_1.data");
    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
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
