use adventofcode_tooling::read_single_string_to_t_vec;

#[must_use]
pub fn part_1(values: &[isize]) -> Option<isize> {
    process(values, |v1, v2| (v1 - v2).abs())
}

#[must_use]
pub fn part_2(values: &[isize]) -> Option<isize> {
    process(values, |v1, v2| (v1 - v2).abs() * ((v1 - v2).abs() + 1) / 2)
}

#[must_use]
pub fn process(values: &[isize], distance: impl Fn(isize, isize) -> isize) -> Option<isize> {
    let (&min, &max) = (values.iter().min()?, values.iter().max()?);
    (min..=max)
        .map(|idx| values.iter().map(|&val| distance(idx, val)).sum::<isize>())
        .min()
}

/// Process solutions for day 7
///
pub fn main() {
    let now = std::time::Instant::now();

    // Read file to a single string
    let values: Vec<isize> = read_single_string_to_t_vec("day_2021_7.data", ',');

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day7_part1() {
        let input = [16_isize, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(part_1(&input), Some(37));
        assert_eq!(part_2(&input), Some(168));
    }
}
