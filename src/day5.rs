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

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<_> = read_lines("day_2021_5.data")?.map(Result::unwrap).collect();

    println!("Part 1: {:?}", part_1(&values));
    // println!("Part 2: {:?}", part_2(&values));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_step1() {
        /*         let values: &[&str] = &[
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(part_1(values).unwrap(), 150); */
        assert!(true);
    }

    #[test]
    fn test_day5_step2() {
        /*         let values: &[&str] = &[
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(part_2(values).unwrap(), 900); */
        assert!(true);
    }
}
