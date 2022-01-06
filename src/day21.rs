use adventofcode_tooling::AocError;

struct DiceIterator(usize);

impl DiceIterator {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn turns(&self) -> usize {
        self.0
    }
}

impl Iterator for DiceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.0 % 100;
        self.0 += 1;
        Some(res + 1)
    }
}

fn part_1(player_1: usize, player_2: usize) -> usize {
    let mut p1 = player_1 - 1;
    let mut p1_score = 0_usize;
    let mut p2 = player_2 - 1;
    let mut p2_score = 0_usize;
    let mut dice = DiceIterator::new();
    while p1_score.lt(&1000) && p2_score.lt(&1000) {
        let sum = dice.by_ref().take(3).sum::<usize>();
        p1 += sum;
        p1 %= 10;
        p1_score += p1 + 1;

        if p1_score.lt(&1000) {
            let sum = dice.by_ref().take(3).sum::<usize>();
            p2 += sum;
            p2 %= 10;
            p2_score += p2 + 1;
        }
    }

    p1_score.min(p2_score) * dice.turns()
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    println!("Part 1: {:?}", part_1(1, 5));
    // println!("Part 2: {:?}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day21_part1() {
        assert_eq!(part_1(4, 8), 739785);
    }

    #[test]
    fn test_day21_part2() {
        // let values: &[usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        // assert_eq!(part_2(values), Ok(5));
        assert!(true)
    }
}
