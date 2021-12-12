use std::time::Instant;

use adventofcode_2021::Matrix2D;
use adventofcode_tooling::{read_lines, AocError};

#[must_use]
pub fn part_1(matrix: &Matrix2D<u8>) -> usize {
    let mut part_matrix = matrix.clone();
    let by_turns = (0..100).map(|_| turn(&mut part_matrix)).collect::<Vec<_>>();
    by_turns.iter().sum()
}

#[must_use]
pub fn part_2(matrix: &Matrix2D<u8>) -> usize {
    let mut part_matrix = matrix.clone();
    let mut turns = 0;
    loop {
        turns += 1;
        if turn(&mut part_matrix) == 100 {
            break;
        }
    }
    turns
}

#[must_use]
pub fn turn(matrix: &mut Matrix2D<u8>) -> usize {
    let width = matrix.width;
    // First, we increase all octopus by 1
    for v in &mut matrix.values {
        *v += 1;
    }

    // We loop until no more octopus need to flash
    loop {
        let need_to_flash: Vec<_> = matrix
            .values
            .iter()
            .enumerate()
            .filter_map(|(u, v)| if *v > 9 { Some(u) } else { None })
            .collect();

        if need_to_flash.is_empty() {
            break;
        }

        for point in need_to_flash {
            let (x, y) = (point % width, point / width);
            // Reset the octopus that has flashed
            if let Some(v) = matrix.values.get_mut(point) {
                *v = 0;
            }

            // x is not left, not right:
            let neighbours = matrix.get_neighbours_coord(x, y, true);

            for position in neighbours {
                if let Some(val) = matrix.values.get_mut(position.1 * width + position.0) {
                    if *val != 0 {
                        *val += 1;
                    }
                }
            }
        }
    }
    bytecount::count(&matrix.values, 0)
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = Instant::now();
    let input_data: Vec<_> = read_lines("day_2021_11.data")?
        .map(Result::unwrap)
        .collect();
    let input_data: Vec<u8> = input_data
        .iter()
        .flat_map(|s| s.chars())
        .map(|s| s.to_digit(10).unwrap())
        .map(|d| u8::try_from(d).unwrap())
        .collect();

    let matrix = {
        let mut matrix = Matrix2D::new(10, 10);
        matrix.values = input_data;
        matrix
    };

    println!("Part 1: {:?}", part_1(&matrix));
    println!("Part 2: {}", part_2(&matrix));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day11_turn() {
        let input_data = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        let input_data: Vec<u8> = input_data
            .lines()
            .flat_map(|s| s.chars())
            .map(|s| s.to_digit(10).unwrap())
            .map(|d| u8::try_from(d).unwrap())
            .collect();

        let mut matrix = {
            let mut matrix = Matrix2D::new(10, 10);
            matrix.values = input_data;
            matrix
        };

        assert_eq!(turn(&mut matrix), 0);
        assert_eq!(turn(&mut matrix), 35);
    }

    #[test]
    fn test_day11_part_1() {
        let input_data = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        let input_data: Vec<u8> = input_data
            .lines()
            .flat_map(|s| s.chars())
            .map(|s| s.to_digit(10).unwrap())
            .map(|d| u8::try_from(d).unwrap())
            .collect();

        let matrix = {
            let mut matrix = Matrix2D::new(10, 10);
            matrix.values = input_data;
            matrix
        };
        assert_eq!(part_1(&matrix), 1656);
    }

    #[test]
    fn test_day11_part_2() {
        let input_data = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        let input_data: Vec<u8> = input_data
            .lines()
            .flat_map(|s| s.chars())
            .map(|s| s.to_digit(10).unwrap())
            .map(|d| u8::try_from(d).unwrap())
            .collect();

        let matrix = {
            let mut matrix = Matrix2D::new(10, 10);
            matrix.values = input_data;
            matrix
        };
        assert_eq!(part_2(&matrix), 195);
    }
}
