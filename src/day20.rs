use std::{fs::read_to_string, path::PathBuf};

use adventofcode_2021::Matrix2D;
use adventofcode_tooling::AocError;

/// Process data for a given step
///
/// # Errors
///
/// When error!
pub fn process(
    matrix: &Matrix2D<bool>,
    algo: &[bool],
    turns: usize,
) -> Result<usize, &'static str> {
    // ERROR HERE: if algo[0] = true, all masked bits are flashing
    let mut res = matrix.clone();
    for _ in 0..turns {
        res = res.improve(algo);
    }
    Ok(res.values.iter().filter(|item| **item).count())
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1(matrix: &Matrix2D<bool>, algo: &[bool]) -> Result<usize, &'static str> {
    process(matrix, algo, 2)
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2(matrix: &Matrix2D<bool>, algo: &[bool]) -> Result<usize, &'static str> {
    process(matrix, algo, 50)
}

/// Process solutions for day 20
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_20.data");

    let data = read_to_string(filepath)?;
    let (algo, matrix) = parse_input(&data).unwrap();

    println!("Part 1: {:?}", part_1(&matrix, &algo));
    println!("Part 2: {:?}", part_2(&matrix, &algo));

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

/// Parse input
///
/// # Errors
///
/// When parsing fails due to format or read error
pub fn parse_input(data_in: &str) -> Result<(Vec<bool>, Matrix2D<bool>), AocError> {
    let lines: Vec<&str> = data_in.lines().collect();

    if lines.len().lt(&3) {
        return Err(AocError::ParsingError);
    }

    let header = lines.get(0).unwrap_or(&"").trim();
    if header.len() != 512 {
        return Err(AocError::ParsingError);
    }

    let ruler: Vec<bool> = header
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect();

    if ruler.len() != 512 {
        return Err(AocError::ParsingError);
    }

    let dim_h = lines.get(2).unwrap_or(&"").trim().len();
    let dim_v = lines.len() - 2;

    let mut matrix: Matrix2D<bool> = Matrix2D::new(dim_h, dim_v);
    matrix.values = lines
        .iter()
        .skip(2)
        .flat_map(|l| l.trim().chars())
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect();

    let extended_matrix = matrix.extend_matrix();

    Ok((ruler, extended_matrix))
}

pub trait Improver {
    fn around_me(&self, x: usize, y: usize) -> usize;
    fn improve(&self, algorithm: &[bool]) -> Matrix2D<bool>;
    fn extend_matrix(&self) -> Matrix2D<bool>;
    fn reduce_matrix(&self) -> Matrix2D<bool>;
}

impl Improver for Matrix2D<bool> {
    fn around_me(&self, x: usize, y: usize) -> usize {
        let mut res = 0;

        // Add upper left
        if matches!(self.neighbour_up_left(x, y), Some(true)) {
            res += 1 << 8;
        }
        // Add upper
        if matches!(self.neighbour_up(x, y), Some(true)) {
            res += 1 << 7;
        }
        // Add upper right
        if matches!(self.neighbour_up_right(x, y), Some(true)) {
            res += 1 << 6;
        } // Add upper left
        if matches!(self.neighbour_left(x, y), Some(true)) {
            res += 1 << 5;
        } // Add upper left
        if matches!(self.x_y_to_idx(x, y), Some(true)) {
            res += 1 << 4;
        } // Add upper left
        if matches!(self.neighbour_right(x, y), Some(true)) {
            res += 1 << 3;
        } // Add upper left
        if matches!(self.neighbour_down_left(x, y), Some(true)) {
            res += 1 << 2;
        } // Add upper left
        if matches!(self.neighbour_down(x, y), Some(true)) {
            res += 1 << 1;
        }
        if matches!(self.neighbour_down_right(x, y), Some(true)) {
            res += 1;
        }
        res
    }

    fn improve(&self, algorithm: &[bool]) -> Matrix2D<bool> {
        // TRICK IS BAD HERE... NOT WORKING
        let det = algorithm[0];
        let mut new_data = vec![];
        new_data.extend(vec![det; (self.width + 6) * 3]);
        for idx_y in 0..self.height {
            new_data.extend(vec![det; 3]);
            for idx_x in 0..self.width {
                if algorithm.get(self.around_me(idx_x, idx_y)) == Some(&true) {
                    new_data.push(true);
                } else {
                    new_data.push(false);
                }
            }
            new_data.extend(vec![det; 3]);
        }
        new_data.extend(vec![det; (self.width + 6) * 3]);
        let mut res = Matrix2D::new(self.width + 6, self.height + 6);
        res.values = new_data;
        res.reduce_matrix()
    }

    fn extend_matrix(&self) -> Matrix2D<bool> {
        let mut extended_matrix = Matrix2D::new(self.width + 6, self.height + 6);
        let mut extended_values = vec![false; self.width * 3 + 18];
        for idx in 0..self.height {
            extended_values.extend(vec![false; 3]);
            extended_values.extend(self.row(idx));
            extended_values.extend(vec![false; 3]);
        }
        extended_values.extend(vec![false; self.width * 3 + 18]);
        extended_matrix.values = extended_values;
        extended_matrix
    }

    fn reduce_matrix(&self) -> Matrix2D<bool> {
        let mut reduced_matrix = Matrix2D::new(self.width - 6, self.height - 6);
        let reduced_values =
            (3..(self.height - 3))
                .map(|idx| self.row(idx))
                .fold(Vec::new(), |mut acc, row| {
                    acc.extend(row[3..row.len() - 3].iter());
                    acc
                });
        reduced_matrix.values = reduced_values;
        reduced_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20_step1() {
        let data = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

        let (algo, matrix) = parse_input(data).unwrap();
        let active = part_1(&matrix, &algo).unwrap();
        assert_eq!(active, 35);
    }

    #[test]
    fn test_day20_step2() {
        let data = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###"#;

        let (algo, matrix) = parse_input(data).unwrap();

        let active = part_2(&matrix, &algo).unwrap();
        assert_eq!(active, 3351);
    }
}
