use std::{fmt::Display, fs::read_to_string, path::PathBuf, str::FromStr};

use adventofcode_2021::Matrix2D;
use adventofcode_tooling::AocError;

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();
        let x = parse_part(&parts, 0)?;
        let y = parse_part(&parts, 1)?;

        Ok(Point { x, y })
    }
}

fn parse_part<T: FromStr>(parts: &[&str], idx: usize) -> Result<T, AocError> {
    parts
        .get(idx)
        .ok_or(AocError::ParsingError)?
        .trim()
        .parse()
        .map_err(|_| AocError::ParsingError)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseStatus {
    Full,
    Empty,
}

impl Display for CaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            CaseStatus::Full => '\u{2588}',
            CaseStatus::Empty => ' ',
        };
        write!(f, "{}", c)
    }
}

#[must_use]
pub fn display_me(matrix: &Matrix2D<CaseStatus>) -> String {
    let rows = matrix.rows();
    let mut s = String::new();
    for row in rows {
        for c in row {
            s.push_str(&c.to_string());
        }
        s.push('\n');
    }
    s
}

#[derive(PartialEq, Clone, Copy, Eq)]
pub enum Fold {
    Horizontal,
    Vertical,
}

#[must_use]
pub fn fold_matrix(
    matrix: &Matrix2D<CaseStatus>,
    fold: Fold,
    coord: usize,
) -> Matrix2D<CaseStatus> {
    let matrix = if fold == Fold::Horizontal {
        std::borrow::Cow::Owned(matrix.transpose())
    } else {
        std::borrow::Cow::Borrowed(matrix)
    };

    let rows = matrix.rows();

    let mut values = vec![];
    for line in 0..coord {
        if let Some(row_up) = rows.get(line) {
            let row_down = rows.get(2 * coord - line);

            if let Some(row_down) = row_down {
                values.extend(row_up.iter().zip(row_down.iter()).map(|(&a, &b)| {
                    if a == CaseStatus::Full || b == CaseStatus::Full {
                        CaseStatus::Full
                    } else {
                        CaseStatus::Empty
                    }
                }));
            } else {
                values.extend(row_up);
            }
        }
    }

    let res = Matrix2D {
        height: coord,
        width: matrix.width,
        values,
    };
    if fold == Fold::Horizontal {
        res.transpose()
    } else {
        res
    }
}

/// Produce this day matrix
///
/// # Errors
///
/// Produces errors when something bad happens in parsing.
pub fn to_matrix(points: &[Point]) -> Result<Matrix2D<CaseStatus>, AocError> {
    let max_x = points
        .iter()
        .map(|p| p.x)
        .max()
        .ok_or(AocError::ParsingError)?;
    let max_y = points
        .iter()
        .map(|p| p.y)
        .max()
        .ok_or(AocError::ParsingError)?;

    let values = {
        let mut values = vec![CaseStatus::Empty; (max_x + 1) * (max_y + 1)];
        for p in points {
            if let Some(pos) = values.get_mut(p.y * (max_x + 1) + p.x) {
                *pos = CaseStatus::Full;
            }
        }
        values
    };

    Ok(Matrix2D {
        width: max_x + 1,
        height: max_y + 1,
        values,
    })
}

#[must_use]
fn part_1(data: &Matrix2D<CaseStatus>, order: Fold, coord: usize) -> usize {
    fold_matrix(data, order, coord)
        .values
        .iter()
        .filter(|&&c| c == CaseStatus::Full)
        .count()
}

#[must_use]
fn part_2(data: &Matrix2D<CaseStatus>, orders: &[(Fold, usize)]) -> String {
    let mut res = data.clone();
    for order in orders {
        res = fold_matrix(&res.clone(), order.0, order.1);
    }
    display_me(&res)
}

/// Process solutions for day 12
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir()?;
    filepath.push("data");
    filepath.push("day_2021_13.data");
    let input_data = read_to_string(filepath)?
        .split("\n\n")
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>();

    let points = input_data[0]
        .lines()
        .map(|s| s.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let matrix = to_matrix(&points)?;

    println!("Part 1: {:?}", part_1(&matrix, Fold::Horizontal, 655));

    let orders = [
        (Fold::Horizontal, 655),
        (Fold::Vertical, 447),
        (Fold::Horizontal, 327),
        (Fold::Vertical, 223),
        (Fold::Horizontal, 163),
        (Fold::Vertical, 111),
        (Fold::Horizontal, 81),
        (Fold::Vertical, 55),
        (Fold::Horizontal, 40),
        (Fold::Vertical, 27),
        (Fold::Vertical, 13),
        (Fold::Vertical, 6),
    ];

    println!("Part 2:");
    println!("{}", part_2(&matrix, &orders));

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day12_part_1() {
        let input_data = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"#;

        let points = input_data
            .lines()
            .map(|s| s.parse::<Point>().unwrap())
            .collect::<Vec<_>>();

        let input_data = to_matrix(&points).unwrap();
        assert_eq!(part_1(&input_data, Fold::Vertical, 7), 17);
    }
}
