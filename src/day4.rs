use std::path::PathBuf;

use adventofcode_2021::Matrix2D;
use adventofcode_tooling::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    Checked,
    Unchecked,
}

type DayMatrix = Matrix2D<(usize, Mark)>;

#[must_use]
pub fn daymatrix_from_str(s: &str) -> DayMatrix {
    let height = s.lines().count();

    let values: Vec<_> = s
        .lines()
        .flat_map(str::split_whitespace)
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .map(|v| (v, Mark::Unchecked))
        .collect();

    let width = values.len() / height;

    DayMatrix {
        width,
        height,
        values,
    }
}

pub trait MarkMatrix {
    fn mark_value_checked(&mut self, val: usize);
    fn has_winning_row(&self) -> bool;
    fn has_winning_col(&self) -> bool;
    fn is_winning(&self) -> bool;
    fn unmarked_sum(&self) -> usize;
}

impl MarkMatrix for DayMatrix {
    fn mark_value_checked(&mut self, val: usize) {
        if let Some(value) = self.values.iter_mut().find(|v| v.0 == val) {
            *value = (val, Mark::Checked);
        }
    }

    #[must_use]
    fn has_winning_row(&self) -> bool {
        self.rows()
            .iter()
            .any(|row| row.iter().all(|&(_, mark)| mark == Mark::Checked))
    }

    #[must_use]
    fn has_winning_col(&self) -> bool {
        self.cols()
            .iter()
            .any(|col| col.iter().all(|&(_, mark)| mark == Mark::Checked))
    }

    #[must_use]
    fn is_winning(&self) -> bool {
        self.has_winning_col() || self.has_winning_row()
    }

    #[must_use]
    fn unmarked_sum(&self) -> usize {
        self.values
            .iter()
            .filter(|&(_, v)| v == &Mark::Unchecked)
            .map(|(u, _)| u)
            .sum()
    }
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1(draw: &[usize], cards: &mut [DayMatrix]) -> Result<usize, AocError> {
    if draw.is_empty() {
        return Err(AocError::ParsingError);
    }
    let mut last_draw = None;

    for val in draw.iter() {
        last_draw = Some(*val);
        cards.iter_mut().for_each(|s| s.mark_value_checked(*val));
        if cards.iter().any(DayMatrix::is_winning) {
            break;
        }
    }

    let matrix_found = cards
        .iter()
        .find(|c| c.is_winning())
        .ok_or(AocError::ParsingError)?;

    Ok(last_draw.ok_or(AocError::ParsingError)? * matrix_found.unmarked_sum())
}

// Process data for ap
///
/// # Errors
///
/// can't produce erro
pub fn part_2(draw: &[usize], cards: &mut [DayMatrix]) -> Result<usize, AocError> {
    let mut cards = cards.to_vec();
    for val in draw.iter() {
        cards.iter_mut().for_each(|s| s.mark_value_checked(*val));
        cards = cards
            .iter()
            .filter(|c| !c.is_winning())
            .map(|c| DayMatrix {
                width: c.width,
                height: c.height,
                values: c.values.clone(),
            })
            .collect();
        if cards.len() == 1 {
            break;
        }
    }

    let mut matrix_found: DayMatrix = cards.pop().ok_or(AocError::ParsingError)?;

    let mut last_draw = None;
    for val in draw.iter() {
        last_draw = Some(*val);
        matrix_found.mark_value_checked(*val);
        if matrix_found.is_winning() {
            break;
        }
    }

    Ok(last_draw.ok_or(AocError::ParsingError)? * matrix_found.unmarked_sum())
}

#[must_use]
pub fn extract_data(data: &str) -> (Vec<usize>, Vec<DayMatrix>) {
    let blocks: Vec<_> = data.split("\n\n").collect();
    let draw: Vec<usize> = blocks[0]
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let cards: Vec<_> = blocks[1..].iter().map(|s| daymatrix_from_str(s)).collect();
    (draw, cards)
}

/// Process solutions for day 3
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::Instant::now();
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_4.data");

    let input_data = std::fs::read_to_string(filepath).unwrap();

    let (draw, mut cards) = extract_data(&input_data);
    let mut cards_clone = cards.clone();
    println!("{}", part_1(&draw, &mut cards).unwrap());
    println!("{}", part_2(&draw, &mut cards_clone).unwrap());
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_step1() {
        let data = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let (draw, mut cards) = extract_data(data);

        assert_eq!(part_1(&draw, &mut cards).unwrap(), 4512);
    }

    #[test]
    fn test_day4_step2() {
        let data = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let (draw, mut cards) = extract_data(data);

        assert_eq!(part_2(&draw, &mut cards).unwrap(), 1924);
    }
}
