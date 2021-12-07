use std::path::PathBuf;

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

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_7.data");

    let input_data = std::fs::read_to_string(filepath)?;
    let values = input_data
        .split(',')
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));

    Ok(())
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
