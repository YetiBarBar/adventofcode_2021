use std::time::Instant;

use adventofcode_2021::{utils::read_lines, Matrix2D};

#[must_use]
pub fn part_1(values: &Matrix2D<usize>) -> usize {
    detect_low_points(values)
        .map(|(x, y)| values.get_x_y(x, y) + 1)
        .sum::<usize>()
}

pub fn detect_low_points(values: &Matrix2D<usize>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..values.width)
        .flat_map(move |x| (0..values.height).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            let val = values.get_x_y(x, y);
            if values.neighbour(x, y, false).iter().all(|v| v.gt(&val)) {
                Some((x, y))
            } else {
                None
            }
        })
}

pub fn part_2(values: &mut Matrix2D<usize>) -> usize {
    let low_points = detect_low_points(values).collect::<Vec<_>>();
    let mut sizes = low_points
        .iter()
        .map(|&(x_low, y_low)| dfs_point(values, x_low, y_low))
        .collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

pub fn dfs_point(values: &mut Matrix2D<usize>, x: usize, y: usize) -> usize {
    let mut dfs = 1_usize;

    values.values[x + y * values.width] = 10;

    if y != 0 && values.neighbour_up(x, y).unwrap_or(9).lt(&9) {
        dfs += dfs_point(values, x, y - 1);
    }
    // Then to the left
    if x != 0 && values.neighbour_left(x, y).unwrap_or(9).lt(&9) {
        dfs += dfs_point(values, x - 1, y);
    }
    // Then to the right
    if values.neighbour_right(x, y).unwrap_or(9).lt(&9) {
        dfs += dfs_point(values, x + 1, y);
    }

    // Then to the bottom
    if values.neighbour_down(x, y).unwrap_or(9).lt(&9) {
        dfs += dfs_point(values, x, y + 1);
    }
    dfs
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let input_data: Vec<String> = read_lines("day_2021_9.data")?.map(Result::unwrap).collect();

    let (width, height) = (input_data.first().unwrap().len(), input_data.len());
    let mut matrix = Matrix2D {
        width,
        height,
        values: input_data
            .iter()
            .flat_map(|s| s.chars())
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect(),
    };

    println!("Part 1: {:?}", part_1(&matrix));
    println!("Part 2: {:?}", part_2(&mut matrix));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day9_part1() {
        let input_data = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        let input_data = input_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let (width, height) = (input_data.first().unwrap().len(), input_data.len());
        let matrix = Matrix2D {
            width,
            height,
            values: input_data
                .iter()
                .flat_map(|s| s.chars())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        };
        assert_eq!(part_1(&matrix), 15);
    }

    #[test]
    fn test_day9_part2() {
        let input_data = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        let input_data = input_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let (width, height) = (input_data.first().unwrap().len(), input_data.len());
        let mut matrix = Matrix2D {
            width,
            height,
            values: input_data
                .iter()
                .flat_map(|s| s.chars())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        };
        assert_eq!(part_2(&mut matrix), 1134);
    }
}
