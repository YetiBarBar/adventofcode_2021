use std::str::FromStr;

/// Provides submarines direction
pub enum Direction {
    Forward,
    Down,
    Up,
}

/// A Submarine command struct
pub struct Command {
    pub direction: Direction,
    pub value: isize,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("Invalid value"),
        }
    }
}

impl FromStr for Command {
    type Err = &'static str;

    /// Convert a String to Command
    ///
    /// # Errors
    ///
    /// Fails if direction is invalid or value is not parsable
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = s.split_whitespace().collect();
        let direction = s[0].parse::<Direction>();
        let value = s[1].parse::<isize>();
        match (direction, value) {
            (Ok(direction), Ok(value)) => Ok(Command { direction, value }),
            (Err(_), Err(_)) => Err("Both invalid"),
            (Err(_), _) => Err("Invalid command"),
            (_, Err(_)) => Err("Invalid value"),
        }
    }
}
