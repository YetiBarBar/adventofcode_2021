use std::{fmt::Display, ops::Add, str::Chars};

use adventofcode_tooling::{read_lines, AocError};

#[derive(Debug, Clone, PartialEq)]
enum SnailNumber {
    Literal(usize),
    Pair(Box<SnailPair>),
}

#[derive(Debug, Clone, PartialEq)]
struct SnailPair(SnailNumber, SnailNumber);

impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNumber::Literal(literal) => write!(f, "{literal}"),
            SnailNumber::Pair(pair) => write!(f, "[{},{}]", &pair.0, &pair.1),
        }
    }
}

impl SnailNumber {
    fn parse(ch: &mut Chars) -> Result<SnailNumber, AocError> {
        match ch.next() {
            Some('[') => {
                let left = Self::parse(ch)?;
                assert_eq!(ch.next(), Some(','));
                let right = Self::parse(ch)?;
                assert_eq!(ch.next(), Some(']'));
                Ok(Self::Pair(Box::new(SnailPair(left, right))))
            }
            Some(x) => {
                let v = x.to_digit(10).ok_or(AocError::ParsingError)?;
                Ok(SnailNumber::Literal(v as usize))
            }
            None => Err(AocError::ParsingError),
        }
    }
}

impl SnailPair {
    fn mag(&self) -> usize {
        3 * match &self.0 {
            SnailNumber::Literal(x) => *x,
            SnailNumber::Pair(p) => p.mag(),
        } + 2 * match &self.1 {
            SnailNumber::Literal(x) => *x,
            SnailNumber::Pair(p) => p.mag(),
        }
    }

    fn explode(&mut self) -> bool {
        self.process_explode(0).is_some()
    }

    fn split(&mut self) -> bool {
        (match &mut self.0 {
            SnailNumber::Literal(n) => {
                if *n > 9 {
                    self.0 = SnailNumber::Pair(Box::new(SnailPair(
                        SnailNumber::Literal(*n / 2),
                        SnailNumber::Literal(*n / 2 + *n % 2),
                    )));
                    true
                } else {
                    false
                }
            }
            SnailNumber::Pair(p) => p.split(),
        } || match &mut self.1 {
            SnailNumber::Literal(n) => {
                if *n > 9 {
                    self.1 = SnailNumber::Pair(Box::new(SnailPair(
                        SnailNumber::Literal(*n / 2),
                        SnailNumber::Literal(*n / 2 + *n % 2),
                    )));
                    true
                } else {
                    false
                }
            }
            SnailNumber::Pair(p) => p.split(),
        })
    }

    fn process_explode(&mut self, depth: usize) -> Option<(bool, usize, usize)> {
        if depth >= 4 {
            if let SnailPair(SnailNumber::Literal(a), SnailNumber::Literal(b)) = self {
                return Some((true, *a, *b));
            }
        }

        let left_eval = match &mut self.0 {
            SnailNumber::Literal(_) => None,
            SnailNumber::Pair(p) => p.process_explode(depth + 1),
        };
        if let Some((first, a, b)) = left_eval {
            if first {
                self.0 = SnailNumber::Literal(0);
            }
            let mut adj_right = &mut self.1;
            let x;
            loop {
                match adj_right {
                    SnailNumber::Literal(y) => {
                        x = *y;
                        break;
                    }
                    SnailNumber::Pair(p) => {
                        adj_right = &mut p.0;
                    }
                }
            }
            *adj_right = SnailNumber::Literal(x + b);
            Some((false, a, 0))
        } else {
            let right_res = match &mut self.1 {
                SnailNumber::Literal(_) => None,
                SnailNumber::Pair(p) => p.process_explode(depth + 1),
            };
            if let Some((first, a, b)) = right_res {
                if first {
                    self.1 = SnailNumber::Literal(0);
                }

                let mut adj_left = &mut self.0;
                let x;
                loop {
                    match adj_left {
                        SnailNumber::Literal(y) => {
                            x = *y;
                            break;
                        }
                        SnailNumber::Pair(p) => {
                            adj_left = &mut p.1;
                        }
                    }
                }
                *adj_left = SnailNumber::Literal(x + a);
                Some((false, 0, b))
            } else {
                None
            }
        }
    }
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
fn part_1(data: &[SnailPair]) -> usize {
    data.iter().cloned().reduce(SnailPair::add).unwrap().mag()
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
fn part_2(data: &[SnailPair]) -> usize {
    data.iter()
        .flat_map(move |a| data.iter().map(move |b| (a.clone(), b.clone())))
        .fold(
            0,
            |max, (a, b)| {
                if a == b {
                    max
                } else {
                    max.max((a + b).mag())
                }
            },
        )
}

impl Add for SnailPair {
    type Output = SnailPair;

    fn add(self, rhs: Self) -> Self::Output {
        let mut s = SnailPair(
            SnailNumber::Pair(Box::new(self)),
            SnailNumber::Pair(Box::new(rhs)),
        );
        loop {
            if !(s.explode() || s.split()) {
                break;
            }
        }
        s
    }
}

/// Process solutions for day 18
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    let input_data = read_lines("day_2021_18.data")?
        .map(Result::unwrap)
        .map(|l| {
            let mut ch = l.chars();
            SnailNumber::parse(&mut ch)
        })
        .map(Result::unwrap)
        .filter_map(|l| {
            if let SnailNumber::Pair(p) = l {
                Some(*p)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input_data));
    println!("Part 2: {}", part_2(&input_data));

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_step1() {
        let raw = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        let values = raw
            .lines()
            .map(|l| {
                let mut ch = l.chars();
                SnailNumber::parse(&mut ch)
            })
            .map(Result::unwrap)
            .filter_map(|l| {
                if let SnailNumber::Pair(p) = l {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(part_1(&values), 4140);
    }

    #[test]
    fn test_day18_step2() {
        let raw = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        let values = raw
            .lines()
            .map(|l| {
                let mut ch = l.chars();
                SnailNumber::parse(&mut ch)
            })
            .map(Result::unwrap)
            .filter_map(|l| {
                if let SnailNumber::Pair(p) = l {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(part_2(&values), 3993);
    }
}
