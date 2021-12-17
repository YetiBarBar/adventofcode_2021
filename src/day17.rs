use std::{path::PathBuf, str::FromStr};

use adventofcode_tooling::AocError;

#[derive(Debug)]
struct Target {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

#[derive(Debug, Clone)]
struct Probe {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
    max_height: isize,
}

impl Target {
    fn is_in(&self, probe: &Probe) -> bool {
        self.xmin.le(&probe.x)
            && self.xmax.ge(&probe.x)
            && self.ymax.ge(&probe.y)
            && self.ymin.le(&probe.y)
    }
}

impl FromStr for Target {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len().lt(&"target area: x=".len()) {
            return Err(AocError::ParsingError);
        }
        let ranges: Vec<_> = s["target area: x=".len()..]
            .split(", y=")
            .map(|s| s.trim().to_string())
            .collect();

        if ranges.len() != 2 {
            return Err(AocError::ParsingError);
        }

        let x = parse_range(&ranges[0])?;
        let y = parse_range(&ranges[1])?;

        Ok(Self {
            xmin: x.0,
            xmax: x.1,
            ymin: y.0,
            ymax: y.1,
        })
    }
}

fn parse_range(input: &str) -> Result<(isize, isize), AocError> {
    let parts = input.split("..").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(AocError::ParsingError);
    }
    let min = parts[0]
        .parse::<isize>()
        .map_err(|_| AocError::ParsingError)?;
    let max = parts[1]
        .parse::<isize>()
        .map_err(|_| AocError::ParsingError)?;
    Ok((min, max))
}

impl Probe {
    fn step(&self, target: &Target) -> Option<Self> {
        let next_target = Probe {
            x: self.x + self.vx,
            y: self.y + self.vy,
            vx: self.vx - self.vx.signum(),
            vy: self.vy - 1,
            max_height: self.max_height.max(self.y + self.vy),
        };

        if next_target.vx.le(&0) && next_target.x.lt(&target.xmin) {
            // No hope to ever reach x_min
            return None;
        }

        if next_target.vx.ge(&0) && next_target.x.gt(&target.xmax) {
            // No hope to ever reach x_max
            return None;
        }

        if next_target.vy.lt(&0) && next_target.y.lt(&target.ymin) {
            // We go deep but are already to low!
            return None;
        }

        // We can push our new point!
        Some(next_target)
    }
}

fn max_height(vx: isize, vy: isize, target: &Target) -> Option<isize> {
    let mut probe = Probe {
        x: 0,
        y: 0,
        vx,
        vy,
        max_height: 0,
    };

    let mut max_h;

    while let Some(new_probe) = probe.step(target) {
        max_h = new_probe.max_height;
        if target.is_in(&new_probe) {
            return Some(max_h);
        }
        probe = new_probe;
    }
    None
}

/// Process solutions for day 17
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_17.data");

    let input = std::fs::read_to_string(&filepath).unwrap();

    let target = Target::from_str(&input)?;

    let mut pos = vec![];

    // Have to find better range here...
    for x_idx in -200..200 {
        for y_idx in -200..200 {
            pos.push((x_idx, y_idx));
        }
    }
    let part_1 = pos
        .iter()
        .filter_map(|(x, y)| max_height(*x, *y, &target))
        .max();

    let part_2 = pos
        .iter()
        .filter_map(|(x, y)| max_height(*x, *y, &target))
        .count();

    println!("{:?}", part_1);
    println!("{:?}", part_2);

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}
