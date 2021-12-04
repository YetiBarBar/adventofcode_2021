use std::{fmt::Display, path::PathBuf, str::FromStr};

use adventofcode_2021::AocError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    Checked,
    Unchecked,
}
#[derive(Clone)]
pub struct MyMatrix {
    dimension_h: usize,
    dimension_v: usize,
    values: Vec<(usize, Mark)>,
}

impl FromStr for MyMatrix {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dimension_v = s.lines().count();

        let values: Vec<_> = s
            .lines()
            .flat_map(|line| line.trim().split_whitespace())
            .map(|s| (s.trim().parse::<usize>().unwrap(), Mark::Unchecked))
            .collect();
        let dimension_h = values.len() / dimension_v;

        Ok(MyMatrix {
            dimension_h,
            dimension_v,
            values,
        })
    }
}

impl Display for MyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for idx in 0..self.dimension_v {
            res.push_str(&format!(
                "{:?}",
                &self.values[idx * self.dimension_h..self.dimension_h * (idx + 1)]
            ));
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}

impl MyMatrix {
    pub fn check_value(&mut self, val: usize) {
        if let Some(value) = self.values.iter_mut().find(|v| v.0 == val) {
            *value = (val, Mark::Checked);
        }
    }

    #[must_use]
    pub fn row(&self, idx: usize) -> Vec<(usize, Mark)> {
        self.values[idx * self.dimension_h..self.dimension_h * (idx + 1)].to_vec()
    }

    #[must_use]
    pub fn col(&self, idx: usize) -> Vec<(usize, Mark)> {
        self.values
            .iter()
            .skip(idx)
            .step_by(self.dimension_h)
            .copied()
            .collect()
    }

    #[must_use]
    pub fn cols(&self) -> Vec<Vec<(usize, Mark)>> {
        (0..self.dimension_h).map(|idx| self.col(idx)).collect()
    }

    #[must_use]
    pub fn rows(&self) -> Vec<Vec<(usize, Mark)>> {
        (0..self.dimension_v).map(|idx| self.row(idx)).collect()
    }

    #[must_use]
    pub fn has_winning_row(&self) -> bool {
        self.rows()
            .iter()
            .any(|row| row.iter().all(|&(_, mark)| mark == Mark::Checked))
    }

    #[must_use]
    pub fn has_winning_col(&self) -> bool {
        self.cols()
            .iter()
            .any(|col| col.iter().all(|&(_, mark)| mark == Mark::Checked))
    }

    #[must_use]
    pub fn is_winning(&self) -> bool {
        self.has_winning_col() || self.has_winning_row()
    }

    #[must_use]
    pub fn unmarked_sum(&self) -> usize {
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
pub fn part_1(tirage: &[usize], cards: &mut Vec<MyMatrix>) -> Result<usize, AocError> {
    if tirage.is_empty() {
        return Err(AocError::ParsingError);
    }
    let mut last_tirage = None;

    for val in tirage.iter() {
        last_tirage = Some(*val);
        cards.iter_mut().for_each(|s| s.check_value(*val));
        if cards.iter().any(MyMatrix::is_winning) {
            break;
        }
    }

    let matrix_found = cards.iter().find(|c| c.is_winning()).unwrap();
    Ok(last_tirage.unwrap() * matrix_found.unmarked_sum())
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2(
    tirage: &[usize],
    cards: &mut Vec<MyMatrix>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut new_cards = cards.clone();
    for val in tirage.iter() {
        // last_tirage = Some(*val);
        new_cards.iter_mut().for_each(|s| s.check_value(*val));
        new_cards = new_cards
            .iter()
            .filter(|c| !c.is_winning())
            .map(|c| MyMatrix {
                dimension_h: c.dimension_h,
                dimension_v: c.dimension_v,
                values: c.values.clone(),
            })
            .collect();
        if new_cards.len() == 1 {
            break;
        }
    }

    let mut matrix_found = MyMatrix {
        dimension_h: new_cards[0].dimension_h,
        dimension_v: new_cards[0].dimension_v,
        values: new_cards[0].values.clone(),
    };

    let mut last_tirage = None;
    for val in tirage.iter() {
        last_tirage = Some(*val);
        matrix_found.check_value(*val);
        if matrix_found.is_winning() {
            break;
        }
    }

    Ok(last_tirage.unwrap() * matrix_found.unmarked_sum())
}

#[must_use]
pub fn extract_data(data: &str) -> (Vec<usize>, Vec<MyMatrix>) {
    let blocks: Vec<_> = data.split("\n\n").collect();

    let tirage: Vec<usize> = blocks[0]
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let cards: Vec<_> = blocks[1..]
        .iter()
        .map(|s| MyMatrix::from_str(s))
        .map(Result::unwrap)
        .collect();
    (tirage, cards)
}

/// Process solutions for day 3
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_4.data");

    let input_data = std::fs::read_to_string(filepath).unwrap();

    let (tirage, mut cards) = extract_data(&input_data);
    println!("{}", part_1(&tirage, &mut cards).unwrap());
    println!("{}", part_2(&tirage, &mut cards).unwrap());

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

        let (tirage, mut cards) = extract_data(data);

        assert_eq!(part_1(&tirage, &mut cards).unwrap(), 4512);
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

        let (tirage, mut cards) = extract_data(data);

        assert_eq!(part_2(&tirage, &mut cards).unwrap(), 1924);
    }
}
