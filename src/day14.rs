use std::{collections::HashMap, path::PathBuf};

use adventofcode_tooling::AocError;

struct Polymere(HashMap<String, usize>);

type PolymereRules = HashMap<String, [String; 2]>;

impl Polymere {
    #[must_use]
    fn new() -> Self {
        Self(HashMap::new())
    }

    #[must_use]
    fn grow(&self, hmap: &PolymereRules) -> Polymere {
        let mut res = HashMap::new();
        for (token, val) in &self.0 {
            if let Some(v) = hmap.get(token) {
                for item in v {
                    *res.entry(item).or_default() += *val;
                }
            }
        }

        Polymere(res.into_iter().map(|(s, v)| (s.to_string(), v)).collect())
    }

    #[must_use]
    fn min_max(&self, first: char, last: char) -> (usize, usize) {
        let folded: HashMap<_, _> = self
            .0
            .iter()
            .flat_map(|(s, count)| s.chars().map(move |ch| (ch, count)))
            .fold(
                [(first, 1_usize), (last, 1)]
                    .into_iter()
                    .collect::<HashMap<char, usize>>(),
                |mut acc, (ch, count)| {
                    *acc.entry(ch).or_default() += count;
                    acc
                },
            );

        // Every char is counted twice!
        let folded: HashMap<char, usize> = folded.iter().map(|(&u, &v)| (u, v / 2)).collect();

        let (min, max) = folded.iter().fold(
            (usize::MAX, usize::MIN),
            |(mut min, mut max), (_, &value)| {
                if value.lt(&min) {
                    min = value;
                }

                if value.gt(&max) {
                    max = value;
                }

                (min, max)
            },
        );

        (min, max)
    }
}

#[must_use]
/// Process this day puzzle
///
/// # Panics
///
/// Panic if data does not provide a message
pub fn process(data: &str, steps: usize) -> usize {
    let (message, hmap) = parse_input(data);
    let first = *message.first().unwrap();
    let last = *message.last().unwrap();
    let message = message
        .windows(2)
        .fold(Polymere::new(), |mut acc, windows| {
            let ent = format!("{}{}", windows[0], windows[1]);
            *acc.0.entry(ent).or_default() += 1_usize;
            acc
        });

    let (min, max) = (1..=steps)
        .fold(message, |acc, _| acc.grow(&hmap))
        .min_max(first, last);

    println!("Min: {min}, Max: {max}");
    max - min
}

#[must_use]
pub fn part_1(data: &str) -> usize {
    process(data, 10)
}

#[must_use]
pub fn part_2(data: &str) -> usize {
    process(data, 40)
}

#[must_use]
fn parse_input<T: AsRef<str>>(input: T) -> (Vec<char>, PolymereRules) {
    let splits = input.as_ref().split("\n\n").collect::<Vec<_>>();
    let base = splits[0].trim().chars().collect::<Vec<_>>();

    let values: PolymereRules = splits[1]
        .lines()
        .map(|s| (s[0..2].to_string(), s.chars().nth(6).unwrap()))
        .map(|(tok, ch)| {
            (tok.to_string(), {
                let mut tok_iter = tok.chars();
                let tok = (
                    tok_iter.next().unwrap_or_default(),
                    tok_iter.next().unwrap_or_default(),
                );
                [format!("{}{}", tok.0, ch), format!("{}{}", ch, tok.1)]
            })
        })
        .collect::<PolymereRules>();
    (base, values)
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
        let data = r"NNCB

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
CN -> C";

        assert_eq!(part_1(data), 1588);
    }
}
