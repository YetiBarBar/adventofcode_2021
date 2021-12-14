use std::{collections::HashMap, fmt::format, io::BufRead, path::PathBuf};

use adventofcode_tooling::{read_lines_to_vec_t, AocError};

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1(data: &str) -> usize {
    let (message, hmap) = parse_input(data);
    let tenth = (1..=10).fold(message, |acc, _| grow(&acc, &hmap));

    let min_max = min_max(&tenth);
    min_max.1 - min_max.0
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2(data: &str) -> usize {
    let (message, hmap) = parse_input(data);

    let tenth = (1..=40).fold(message, |acc, idx| {
        println!("{}", idx);
        grow(&acc, &hmap)
    });
    // tenth.len()
    let min_max = min_max(&tenth);
    min_max.1 - min_max.0
}

pub fn parse_input<T: AsRef<str>>(input: T) -> (Vec<char>, HashMap<String, char>) {
    let splits = input.as_ref().split("\n\n").collect::<Vec<_>>();
    let base = splits[0].trim().chars().collect::<Vec<_>>();

    let values: HashMap<String, char> = splits[1]
        .lines()
        .map(|s| (s[0..2].to_string(), s.chars().nth(6).unwrap()))
        .collect();
    (base, values)
}

pub fn grow(input: &[char], hmap: &HashMap<String, char>) -> Vec<char> {
    let mut res = input.windows(2).fold(Vec::new(), |mut acc, ch| {
        let item = format!("{}{}", ch[0], ch[1]);
        acc.push(ch[0]);
        if let Some(token) = hmap.get(&item) {
            acc.push(*token);
        }
        acc
    });
    res.push(*input.last().unwrap());
    res
}

pub fn min_max(input: &[char]) -> (usize, usize) {
    let mut folded = HashMap::new();
    for ch in input {
        folded
            .entry(ch)
            .and_modify(|val| {
                *val += 1;
            })
            .or_insert(1_usize);
    }

    let folded = folded.iter().fold((None, None), |mut acc, (_, value)| {
        let (cur_min, cur_max) = (acc.0, acc.1);
        if cur_min.is_none() {
            acc.0 = Some(value);
        } else {
            if let Some(cur_min) = cur_min {
                if value.lt(cur_min) {
                    acc.0 = Some(value);
                }
            }
        }
        if cur_max.is_none() {
            acc.1 = Some(value);
        } else {
            if let Some(cur_max) = cur_max {
                if value.gt(cur_max) {
                    acc.1 = Some(value);
                }
            }
        }
        acc
    });
    (folded.0.unwrap().clone(), folded.1.unwrap().clone())
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
    /* let values = read_lines_to_vec_t("day_2021_1.data");
    println!("Part 1: {:?}", part_1(&values)); */
    //println!("Part 2: {:?}", part_2(&values));
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

        let (message, hmap) = parse_input(&data);
        let v1 = grow(&message, &hmap);
        let v2 = grow(&v1, &hmap);
        let v3 = grow(&v2, &hmap);
        let v4 = grow(&v3, &hmap);
        assert_eq!(v1.iter().collect::<String>(), "NCNBCHB");
        assert_eq!(v2.iter().collect::<String>(), "NBCCNBBBCBHCB");
        assert_eq!(v3.iter().collect::<String>(), "NBBBCNCCNBBNBNBBCHBHHBCHB");
        assert_eq!(
            v4.iter().collect::<String>(),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
        assert_eq!(part_1(&data), 1588);
    }
}
