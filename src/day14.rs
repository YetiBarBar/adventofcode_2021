use std::{collections::HashMap, fmt::format, io::BufRead, iter::empty, path::PathBuf};

use adventofcode_tooling::{read_lines_to_vec_t, AocError};

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn process(data: &str, steps: usize) -> usize {
    let (message, hmap) = parse_input(data);
    let first = *message.first().unwrap();
    let last = *message.last().unwrap();
    let message = message.windows(2).fold(HashMap::new(), |mut acc, windows| {
        let ent = format!("{}{}", windows[0], windows[1]);
        acc.entry(ent)
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(1_usize);
        acc
    });

    let (min, max) = min_max(
        &(1..=steps).fold(message, |acc, _| grow(&acc, &hmap)),
        first,
        last,
    );

    max - min
}

pub fn part_1(data: &str) -> usize {
    process(data, 10)
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2(data: &str) -> usize {
    process(data, 40)
}

pub fn parse_input<T: AsRef<str>>(input: T) -> (Vec<char>, HashMap<String, Vec<String>>) {
    let splits = input.as_ref().split("\n\n").collect::<Vec<_>>();
    let base = splits[0].trim().chars().collect::<Vec<_>>();

    let values = splits[1]
        .lines()
        .map(|s| (s[0..2].to_string(), s.chars().nth(6).unwrap()))
        .map(|(tok, ch)| {
            (tok.to_string(), {
                let tok = tok.chars().collect::<Vec<_>>();
                [tok[0], ch, tok[1]]
                    .windows(2)
                    .map(|t| t.iter().collect::<String>())
                    .collect()
            })
        })
        .collect::<HashMap<String, Vec<String>>>();
    (base, values)
}

pub fn grow(
    input: &HashMap<String, usize>,
    hmap: &HashMap<String, Vec<String>>,
) -> HashMap<String, usize> {
    let mut res = HashMap::new();
    for (token, val) in input.iter() {
        let empty = vec![];
        let v = hmap.get(token).unwrap_or(&empty).to_owned();

        for item in v {
            res.entry(item)
                .and_modify(|e| {
                    *e += val;
                })
                .or_insert(*val);
        }
    }

    res.into_iter().map(|(s, v)| (s, v)).collect()
}

pub fn min_max(input: &HashMap<String, usize>, first: char, last: char) -> (usize, usize) {
    let mut folded = HashMap::new();

    for ch in input {
        for c in ch.0.chars() {
            folded
                .entry(c)
                .and_modify(|val| {
                    *val += ch.1;
                })
                .or_insert(*ch.1);
        }
    }
    *folded.entry(first).or_insert(0) += 1;
    *folded.entry(last).or_insert(0) += 1;
    let res: (Option<&usize>, Option<&usize>) =
        folded.iter().fold((None, None), |mut acc, (_, value)| {
            let (cur_min, cur_max) = (acc.0, acc.1);
            if cur_min.is_none() {
                acc.0 = Some(value);
            } else if let Some(cur_min) = cur_min {
                if value.lt(cur_min) {
                    acc.0 = Some(value);
                }
            }
            if cur_max.is_none() {
                acc.1 = Some(value);
            } else if let Some(cur_max) = cur_max {
                if value.gt(cur_max) {
                    acc.1 = Some(value);
                }
            }
            acc
        });

    (*res.0.unwrap_or(&0) / 2, *res.1.unwrap_or(&0) / 2)
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_14.data");

    let input_data = std::fs::read_to_string(filepath).unwrap();

    println!("{}", part_1(&input_data));
    println!("{}", part_2(&input_data));

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_step1() {
        let data = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

        assert_eq!(part_1(&data), 1588);
    }
}
