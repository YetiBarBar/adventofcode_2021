use hashbrown::HashMap;
use std::{fmt::Debug, str::FromStr};

use adventofcode_tooling::{read_lines_to_vec_t, AocError};

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    #[must_use]
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();
        let x = parse_part(&parts, 0)?;
        let y = parse_part(&parts, 1)?;

        Ok(Point::new(x, y))
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

#[derive(Debug, PartialEq, Eq)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl FromStr for Segment {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("->").collect();

        let a = parse_part(&parts, 0)?;
        let b = parse_part(&parts, 1)?;

        Ok(Segment::new(a, b))
    }
}

impl Segment {
    #[must_use]
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    #[must_use]
    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    #[must_use]
    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    #[must_use]
    pub fn is_diagonal(&self) -> bool {
        let d_y = self.a.y - self.b.y;
        let d_x = self.a.x - self.b.x;
        d_y.abs() == d_x.abs()
    }

    #[must_use]
    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    #[must_use]
    pub fn is_horizontal_vertical_or_diagonal(&self) -> bool {
        self.is_diagonal() || self.is_horizontal_or_vertical()
    }

    #[must_use]
    pub fn points(&self) -> Vec<Point> {
        let mut res = self.h_points();
        res.extend(self.v_points());
        res.extend(self.diag_points());
        res
    }

    #[must_use]
    pub fn h_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let (min, max) = match self.a.x.cmp(&self.b.x) {
                std::cmp::Ordering::Greater => (self.b.x, self.a.x),
                _ => (self.a.x, self.b.x),
            };

            (min..=max).map(|x| Point::new(x, self.b.y)).collect()
        } else {
            vec![]
        }
    }

    #[must_use]
    pub fn v_points(&self) -> Vec<Point> {
        if self.is_vertical() {
            let (min, max) = match self.a.y.cmp(&self.b.y) {
                std::cmp::Ordering::Greater => (self.b.y, self.a.y),
                _ => (self.a.y, self.b.y),
            };

            (min..=max).map(|y| Point::new(self.b.x, y)).collect()
        } else {
            vec![]
        }
    }

    #[must_use]
    pub fn diag_points(&self) -> Vec<Point> {
        if self.is_diagonal() {
            let (min_x, max_x) = match self.a.x.cmp(&self.b.x) {
                std::cmp::Ordering::Greater => (self.b.x, self.a.x),
                _ => (self.a.x, self.b.x),
            };
            let (min_y, max_y) = match self.a.y.cmp(&self.b.y) {
                std::cmp::Ordering::Greater => (self.b.y, self.a.y),
                _ => (self.a.y, self.b.y),
            };

            let p = Point::new(min_x, min_y);
            if p == self.a || p == self.b {
                (min_x..=max_x)
                    .zip(min_y..=max_y)
                    .map(|(x, y)| Point::new(x, y))
                    .collect()
            } else {
                (min_x..=max_x)
                    .zip((min_y..=max_y).rev())
                    .map(|(x, y)| Point::new(x, y))
                    .collect()
            }
        } else {
            vec![]
        }
    }
}
/// Process data for a given step
///
/// # Errors
///
/// TBD
pub fn part_1(data: &[Segment]) -> usize {
    process(data, Segment::is_horizontal_or_vertical)
}

/// Process data for a given step
///
/// # Errors
///
/// TBD
pub fn part_2(data: &[Segment]) -> usize {
    process(data, Segment::is_horizontal_vertical_or_diagonal)
}

/// Process data for a given step
///
/// # Errors
///
/// TBD
pub fn process(data: &[Segment], seg_condition: impl Fn(&Segment) -> bool) -> usize {
    let mut hmap: HashMap<_, usize> = HashMap::new();
    data.iter()
        .filter(|s| seg_condition(s))
        .flat_map(Segment::points)
        .for_each(|point| {
            let pos = (point.x, point.y);
            *hmap.entry(pos).or_default() += 1;
        });
    hmap.iter().filter(|&(_, val)| val.ge(&2)).count()
}

/// Process solutions for day 5
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() {
    let now = std::time::Instant::now();
    let values: Vec<Segment> = read_lines_to_vec_t("day_2021_5.data");

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_segment_from_str() {
        let segment = "0,9 -> 5,9";
        assert_eq!(
            Segment::from_str(segment).unwrap(),
            Segment::new(Point::new(0, 9), Point::new(5, 9))
        );
    }

    #[test]
    fn test_day5_step1() {
        let values = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let input: Vec<_> = values
            .lines()
            .map(|l| Segment::from_str(l).unwrap())
            .collect();
        assert_eq!(part_1(&input), 5);
    }
    #[test]
    fn test_day5_step2() {
        let values = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let input: Vec<_> = values
            .lines()
            .map(|l| Segment::from_str(l).unwrap())
            .collect();
        assert_eq!(part_2(&input), 12);
    }
}
