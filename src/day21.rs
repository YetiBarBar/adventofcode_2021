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

struct Player {
    position: usize,
    score: usize,
}

impl Player {
    pub fn new(position: usize) -> Self {
        Player {
            position: position - 1,
            score: 0,
        }
    }

    pub fn advance_by(&mut self, move_val: usize) {
        self.position += move_val;
        self.position %= 10;
        self.score += self.position + 1;
    }

    pub fn score(&self) -> usize {
        self.score
    }
}

fn part_1(player_1: usize, player_2: usize) -> usize {
    let mut player1 = Player::new(player_1);
    let mut player2 = Player::new(player_2);

    let mut dice = DiceIterator::new();

    loop {
        player1.advance_by(dice.by_ref().take(3).sum());
        if player1.score().ge(&1000) {
            break;
        }
        player2.advance_by(dice.by_ref().take(3).sum());
        if player2.score().ge(&1000) {
            break;
        }
    }

    player1.score().min(player2.score()) * dice.turns()
}

/// Process solutions for day 21
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
