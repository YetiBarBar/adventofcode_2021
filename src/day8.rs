use std::collections::HashMap;

use adventofcode_2021::utils::read_lines;

#[must_use]
pub fn part_1(values: &[String]) -> usize {
    values
        .iter()
        .map(|s| {
            s.split('|')
                .nth(1)
                .unwrap_or("")
                .split(' ')
                .filter(|item| {
                    let l = item.len();
                    [2, 3, 4, 7].contains(&l)
                })
                .count()
        })
        .sum()
}

#[must_use]
pub fn extract_str_len(input: &str, size: usize) -> Vec<Vec<char>> {
    input
        .split('|')
        .next()
        .unwrap_or(&"")
        .split(' ')
        .filter(|item| item.len() == size)
        .map(|s| {
            let mut r = s.chars().collect::<Vec<char>>();
            r.sort_unstable();
            r
        })
        .collect()
}

#[must_use]
pub fn solve(input: &str) -> HashMap<String, usize> {
    /*
       If length is, number is:
       * 2 => 1,
       * 3 => 7,
       * 4 => 4,
       * 5 => case 5!
       * 6 => case 6!
       * 7 => 8

       Case 5:
       if 2 letters common with 1 => 3,
       else if 3 letters common with 4 => 5,
       else 2

       Case 6:
       if 'only 1 letter' common with 1 => 6
       else if 4 letter common with 4 => 9,
       else 0

    */
    let mut hmap = HashMap::new();
    let s = input
        .split(' ')
        //.map(|s| s.to_string())
        .collect::<Vec<_>>();
    let my_default = ""; //.to_string();
    let one = s.iter().find(|s| s.len() == 2).unwrap_or(&my_default);
    let seven = s.iter().find(|s| s.len() == 3).unwrap_or(&my_default);
    let four = s.iter().find(|s| s.len() == 4).unwrap_or(&my_default);
    let eight = s.iter().find(|s| s.len() == 7).unwrap_or(&my_default);
    hmap.insert(sorted_str(one), 1);
    hmap.insert(sorted_str(seven), 7);
    hmap.insert(sorted_str(four), 4);
    hmap.insert(sorted_str(eight), 8);

    let fives: Vec<_> = s.iter().filter(|s| s.len() == 5).collect();
    for data in fives {
        hmap.insert(sorted_str(data), is_three_five_two(data, one, four));
    }

    let sixs: Vec<_> = s.iter().filter(|s| s.len() == 6).collect();
    for data in sixs {
        hmap.insert(sorted_str(data), is_one_six_nine(data, one, four));
    }
    hmap
}

#[must_use]
pub fn sorted_str<T: AsRef<str>>(s: T) -> String {
    let mut s = s.as_ref().chars().collect::<Vec<char>>();
    s.sort_unstable();
    s.iter().collect()
}

#[must_use]
pub fn is_one_six_nine(input: &str, one: &str, four: &str) -> usize {
    if input.chars().filter(|&c| one.contains(c)).count() == 1 {
        6
    } else if input.chars().filter(|&c| four.contains(c)).count() == 4 {
        9
    } else {
        0
    }
}

#[must_use]
pub fn is_three_five_two(input: &str, one: &str, four: &str) -> usize {
    if input.chars().filter(|&c| one.contains(c)).count() == 2 {
        3
    } else if input.chars().filter(|&c| four.contains(c)).count() == 3 {
        5
    } else {
        2
    }
}

#[must_use]
pub fn process(values: &[isize], distance: impl Fn(isize, isize) -> isize) -> Option<isize> {
    let (&min, &max) = (values.iter().min()?, values.iter().max()?);
    (min..=max)
        .map(|idx| values.iter().map(|&val| distance(idx, val)).sum::<isize>())
        .min()
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read file to a single string

    let input_data: Vec<_> = read_lines("day_2021_8.data")?
        .map(Result::unwrap)
        //.map(|s| s.to_string())
        .collect();

    println!("Part 1: {:?}", part_1(&input_data));
    println!("Part 2: {:?}", part_2(&input_data));

    Ok(())
}

#[must_use]
pub fn part_2(input: &[String]) -> usize {
    input.iter().map(|s| solve_line(s)).sum()
}

#[must_use]
fn solve_line(s: &str) -> usize {
    let parts: Vec<_> = s.split('|').collect();
    let dict = solve(parts[0]);
    let res: Vec<_> = parts[1]
        .split_ascii_whitespace()
        .map(|s| {
            let s = sorted_str(s);
            dict.get(&s).unwrap()
        })
        .collect();

    res[0] * 1000 + res[1] * 100 + res[2] * 10 + res[3]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day8_part1() {
        let data = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

        let values: Vec<_> = data.lines().map(|s| s.to_string()).collect();

        assert_eq!(part_1(&values), 26);
        assert_eq!(part_2(&values), 61229);
    }
}
