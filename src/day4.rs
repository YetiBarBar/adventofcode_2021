use adventofcode_2021::utils::read_lines;

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1<T: AsRef<str>>(_data: &[T]) -> Result<isize, Box<dyn std::error::Error>> {
    todo!()
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2<T: AsRef<str>>(_data: &[T]) -> Result<isize, Box<dyn std::error::Error>> {
    todo!()
}

/// Process solutions for day 3
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<_> = read_lines("day_2021_4.data")?.map(Result::unwrap).collect();

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_step1() {
        let values: &[&str] = &[
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(part_1(values).unwrap(), 198);
    }

    #[test]
    fn test_day4_step2() {
        let values: &[&str] = &[
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(part_2(values).unwrap(), 230);
    }
}
