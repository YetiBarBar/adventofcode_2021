use std::time::Instant;

use adventofcode_2021::{utils::read_lines, AocError};

#[must_use]
pub fn part_1(values: &[String]) -> usize {
    values
        .iter()
        .filter_map(|s| {
            if let ExpressionStatus::Corrupted(c) = process(s) {
                Some(c)
            } else {
                None
            }
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

#[must_use]
pub fn part_2(values: &[String]) -> Option<u128> {
    let mut tab = values
        .iter()
        .filter_map(|s| {
            if let ExpressionStatus::Incomplete(v) = process(s) {
                Some(v)
            } else {
                None
            }
        })
        .map(|v| {
            v.iter().rev().fold(0, |mut acc: u128, &c| {
                acc *= 5;
                acc += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
                acc
            })
        })
        .collect::<Vec<_>>();
    tab.sort_unstable();

    let len = tab.len();
    if len & 1 != 1 {
        println!("Number of incomplete lines is assumed to be even.");
        println!("Result is probably wrong");
    }
    let middle = tab.len() / 2;

    tab.get(middle).copied()
}

pub enum ExpressionStatus {
    Valid,
    Incomplete(Vec<char>),
    Corrupted(char),
}

pub fn process(expr: &str) -> ExpressionStatus {
    let mut stack = Vec::<char>::new();

    for c in expr.chars() {
        if c == '}' {
            if stack.last() != Some(&'{') {
                return ExpressionStatus::Corrupted(c);
            }
            stack.pop();
        } else if c == ']' {
            if stack.last() != Some(&'[') {
                return ExpressionStatus::Corrupted(c);
            }
            stack.pop();
        } else if c == ')' {
            if stack.last() != Some(&'(') {
                return ExpressionStatus::Corrupted(c);
            }
            stack.pop();
        } else if c == '>' {
            if stack.last() != Some(&'<') {
                return ExpressionStatus::Corrupted(c);
            }
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    if !stack.is_empty() {
        return ExpressionStatus::Incomplete(stack);
    }
    ExpressionStatus::Valid
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = Instant::now();
    let input_data: Vec<String> = read_lines("day_2021_10.data")?
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {:?}", part_1(&input_data));
    println!("Part 2: {:?}", part_2(&input_data));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day10_part1() {
        let input_data = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        let input_data = input_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(part_1(&input_data), 26397);
    }

    #[test]
    fn test_day10_part2() {
        let input_data = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        let input_data = input_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(part_2(&input_data), Some(288957));
    }
}
