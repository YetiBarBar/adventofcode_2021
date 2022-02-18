use adventofcode_2021::submarine::{Command, Direction};
use adventofcode_tooling::{read_lines, AocError};

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_1(data: &[Command]) -> isize {
    let (horizontal, depth) = data
        .iter()
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
    horizontal * depth
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_2(data: &[Command]) -> isize {
    let (horizontal, depth, _) =
        data.iter()
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
    horizontal * depth
}

/// Process solutions for day 2
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    let values: Vec<_> = read_lines("day_2021_2.data")?
        .map(Result::unwrap)
        .map(|s| s.parse::<Command>())
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {}", part_1(&values));
    println!("Part 2: {}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
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
        let values = values
            .iter()
            .map(|s| s.parse::<Command>())
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        assert_eq!(part_1(&values), 150);
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
        let values = values
            .iter()
            .map(|s| s.parse::<Command>())
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        assert_eq!(part_2(&values), 900);
    }
}
