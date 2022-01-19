use adventofcode_tooling::AocError;
use std::collections::HashMap;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Player {
    position: usize,
    score: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Game {
    players: [Player; 2],
}
impl Game {
    fn advance(&self, player: usize, roll: usize) -> Self {
        let mut next = *self;
        next.players[player].advance_by(roll);
        next
    }
}

impl Player {
    pub fn new(position: usize) -> Self {
        Player {
            position: position - 1,
            score: 0,
        }
    }

    pub fn advance_by(&mut self, move_index: usize) {
        self.position += move_index;
        self.position %= 10;
        self.score += self.position + 1;
    }

    pub fn score(&self) -> usize {
        self.score
    }
}

fn part_1(player1: usize, player2: usize) -> usize {
    let mut player1 = Player::new(player1);
    let mut player2 = Player::new(player2);

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

fn part_2(player1: usize, player2: usize) -> usize {
    let players = [Player::new(player1), Player::new(player2)];
    let mut wins = [0_usize, 0];
    let mut games = HashMap::from([(Game { players }, 1_usize)]);

    let rolls: Vec<_> = (1..=3)
        .flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)))
        .collect();

    for player in (0..=1).cycle() {
        let mut next = HashMap::new();
        for &roll in &rolls {
            for (game, universes) in &games {
                let advanced = game.advance(player, roll);
                if advanced.players[player].score >= 21 {
                    wins[player] += universes;
                } else {
                    *next.entry(advanced).or_default() += universes;
                }
            }
        }
        games = next;
        if games.is_empty() {
            break;
        }
    }
    let [p1_wins, p2_wins] = wins;
    p1_wins.max(p2_wins)
}

/// Process solutions for day 21
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    println!("Part 1: {:?}", part_1(1, 5));
    println!("Part 2: {:?}", part_2(1, 5));
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
