use std::cmp::Ordering;

use adventofcode_2021::utils::read_lines;

#[derive(Debug, Default)]
struct BitCounter {
    zeroes: usize,
    ones: usize,
}

impl BitCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_one(&mut self) {
        self.ones += 1;
    }

    pub fn add_zero(&mut self) {
        self.zeroes += 1;
    }

    pub fn decide_gamma(&self) -> u8 {
        match self.zeroes.cmp(&self.ones) {
            Ordering::Greater => 0,
            _ => 1,
        }
    }

    pub fn decide_alpha(&self) -> u8 {
        match self.zeroes.cmp(&self.ones) {
            Ordering::Greater => 1,
            _ => 0,
        }
    }
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1<T: AsRef<str>>(data: &[T]) -> Result<isize, Box<dyn std::error::Error>> {
    let str_len = data[1].as_ref().len();
    let mut initial_vec = Vec::new();
    for _ in 0..str_len {
        initial_vec.push(BitCounter::new());
    }

    data.iter().map(std::convert::AsRef::as_ref).for_each(|s| {
        s.chars().enumerate().for_each(|(pos, c)| {
            if c == '0' {
                initial_vec[pos].add_zero();
            } else {
                initial_vec[pos].add_one();
            }
        });
    });

    let gamma = evaluate(&initial_vec, BitCounter::decide_alpha);
    let epsilon = evaluate(&initial_vec, BitCounter::decide_gamma);

    Ok(gamma * epsilon)
}

fn evaluate(values: &[BitCounter], decider: impl Fn(&BitCounter) -> u8) -> isize {
    values.iter().map(decider).fold(0, |mut acc, bit| {
        acc <<= 1;
        if bit == 1 {
            acc += 1;
        }
        acc
    })
}

pub enum Rule {
    Oxygen,
    Co2,
}

#[must_use]
pub fn partitions(data: &[String], idx: usize, rule: &Rule) -> Vec<String> {
    let (p0, p1) = (partitioner(data, idx, '0'), partitioner(data, idx, '1'));

    match rule {
        Rule::Oxygen => {
            if p0.len() > p1.len() {
                p0
            } else {
                p1
            }
        }
        Rule::Co2 => {
            if p1.len() < p0.len() {
                p1
            } else {
                p0
            }
        }
    }
}

fn partitioner(data: &[String], idx: usize, value: char) -> Vec<String> {
    data.iter()
        .filter(|s| s.chars().nth(idx) == Some(value))
        .cloned()
        .collect()
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2<T: AsRef<str>>(data: &[T]) -> Result<isize, Box<dyn std::error::Error>> {
    let str_len = data[1].as_ref().len();
    let mut initial_vec = Vec::new();
    for _ in 0..str_len {
        initial_vec.push(BitCounter::new());
    }

    let mut result_oxygen = data
        .iter()
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<_>>();
    for idx in 0..str_len {
        if result_oxygen.len() != 1 {
            result_oxygen = partitions(&result_oxygen, idx, &Rule::Oxygen);
        }
    }

    let mut result_co2 = data
        .iter()
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<_>>();

    for idx in 0..str_len {
        if result_co2.len() != 1 {
            result_co2 = partitions(&result_co2, idx, &Rule::Co2);
        }
    }

    let co2 = bit_str_to_isize(&result_co2);
    let oxygen = bit_str_to_isize(&result_oxygen);

    Ok(oxygen * co2)
}

fn bit_str_to_isize(input: &[String]) -> isize {
    input[0].chars().fold(0, |mut acc, bit| {
        acc <<= 1;
        if bit == '1' {
            acc += 1;
        }
        acc
    })
}

/// Process solutions for day 3
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<_> = read_lines("day_2021_3.data")?.map(Result::unwrap).collect();

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_step1() {
        let values: &[&str] = &[
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(part_1(values).unwrap(), 198);
    }

    #[test]
    fn test_day3_step2() {
        let values: &[&str] = &[
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(part_2(values).unwrap(), 230);
    }
}
