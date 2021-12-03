use adventofcode_2021::{
    submarine::{Command, Direction},
    utils::read_lines,
};

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1<T: AsRef<str>>(data: &[T]) -> Result<isize, Box<dyn std::error::Error>> {
    let (horizontal, depth) = data
        .iter()
        .map(|s| s.as_ref().parse::<Command>())
        .map(Result::unwrap)
        .fold((0, 0), |(mut horizontal, mut depth), dir| {
            match dir.direction {
                Direction::Forward => {
                    horizontal += dir.value;
                }
                Direction::Down => {
                    depth += dir.value;
                }
                Direction::Up => {
                    depth -= dir.value;
                }
            }

            (horizontal, depth)
        });
    Ok(horizontal * depth)
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2<T: AsRef<str>>(data: &[T]) -> Result<isize, Box<dyn std::error::Error>> {
    let (horizontal, depth, _) = data
        .iter()
        .map(|s| s.as_ref().parse::<Command>())
        .map(Result::unwrap)
        .fold((0, 0, 0), |(mut horizontal, mut depth, mut aim), dir| {
            match dir.direction {
                Direction::Forward => {
                    horizontal += dir.value;
                    depth += dir.value * aim;
                }
                Direction::Down => {
                    aim += dir.value;
                }
                Direction::Up => {
                    aim -= dir.value;
                }
            }
            (horizontal, depth, aim)
        });
    Ok(horizontal * depth)
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<_> = read_lines("day_2021_2.data")?.map(Result::unwrap).collect();

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_step1() {
        let values: &[&str] = &[
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(part_1(values).unwrap(), 150);
    }

    #[test]
    fn test_day2_step2() {
        let values: &[&str] = &[
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(part_2(values).unwrap(), 900);
    }
}
