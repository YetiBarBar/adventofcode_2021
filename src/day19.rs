use adventofcode_tooling::AocError;
use hashbrown::HashSet;
use itertools::Itertools;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct BeaconRelativePos(isize, isize, isize);

impl From<&(isize, isize, isize)> for BeaconRelativePos {
    fn from(beacon: &(isize, isize, isize)) -> Self {
        BeaconRelativePos(beacon.0, beacon.1, beacon.2)
    }
}

impl BeaconRelativePos {
    fn orientate(&self, idx: usize) -> Option<(isize, isize, isize)> {
        let &BeaconRelativePos(x, y, z) = self;
        match idx {
            0 => Some((x, y, z)),
            1 => Some((x, z, -y)),
            2 => Some((x, -y, -z)),
            3 => Some((x, -z, y)),
            4 => Some((y, x, -z)),
            5 => Some((y, z, x)),
            6 => Some((y, -x, z)),
            7 => Some((y, -z, -x)),
            8 => Some((z, x, y)),
            9 => Some((z, y, -x)),
            10 => Some((z, -x, -y)),
            11 => Some((z, -y, x)),
            12 => Some((-x, y, -z)),
            13 => Some((-x, z, y)),
            14 => Some((-x, -y, z)),
            15 => Some((-x, -z, -y)),
            16 => Some((-y, x, z)),
            17 => Some((-y, z, -x)),
            18 => Some((-y, -x, -z)),
            19 => Some((-y, -z, x)),
            20 => Some((-z, x, -y)),
            21 => Some((-z, y, x)),
            22 => Some((-z, -x, y)),
            23 => Some((-z, -y, -x)),
            _ => unreachable!(),
        }
    }
}

impl FromStr for BeaconRelativePos {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.split(',').map(str::trim).collect::<Vec<_>>();
        if coords.len() != 3 {
            return Err(AocError::ParsingError);
        }

        let c0 = coords[0]
            .parse::<isize>()
            .map_err(|_| AocError::ParsingError)?;
        let c1 = coords[1]
            .parse::<isize>()
            .map_err(|_| AocError::ParsingError)?;
        let c2 = coords[2]
            .parse::<isize>()
            .map_err(|_| AocError::ParsingError)?;

        Ok(Self(c0, c1, c2))
    }
}

#[derive(Debug, Clone)]
struct Probe {
    id: usize,
    beacons: HashSet<BeaconRelativePos>,
}

impl Probe {
    fn id(&self) -> usize {
        self.id
    }

    fn orientate_all_beacons(&self, idx: usize) -> HashSet<(isize, isize, isize)> {
        self.beacons
            .iter()
            .filter_map(|beacon| beacon.orientate(idx))
            .collect()
    }

    fn try_merge(&mut self, other: &Probe) -> Option<(isize, isize, isize)> {
        // We only need one direction for self and 24 for other
        for idx in 0..24 {
            let rotated = other.orientate_all_beacons(idx);
            let distances = self
                .beacons
                .iter()
                .cartesian_product(&rotated)
                .map(|(a, b)| (a.0 - b.0, a.1 - b.1, a.2 - b.2))
                .collect::<Vec<_>>();

            for (dx, dy, dz) in distances {
                let translated = rotated
                    .iter()
                    .map(|&(x, y, z)| (x + dx, y + dy, z + dz))
                    .collect::<Vec<_>>();
                if translated
                    .iter()
                    .filter(|&&beacon| {
                        self.beacons
                            .contains(&BeaconRelativePos(beacon.0, beacon.1, beacon.2))
                    })
                    .count()
                    .ge(&12)
                {
                    self.beacons.extend(translated.iter().map(|b| b.into()));
                    return Some((dx, dy, dz));
                }
            }
        }
        None
    }
}

impl FromStr for Probe {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(str::trim).collect::<Vec<_>>();
        if lines.len() < 2 {
            return Err(AocError::ParsingError);
        }
        let id = lines
            .get(0)
            .and_then(|s| s.split(' ').nth(2))
            .and_then(|val| val.parse::<usize>().ok())
            .ok_or(AocError::ParsingError)?;

        let beacons = lines
            .iter()
            .skip(1)
            .map(|line| line.parse::<BeaconRelativePos>())
            .collect::<Result<_, _>>()?;

        Ok(Probe { id, beacons })
    }
}

/// Process solutions for day 19
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_19.data");

    let mut probes = std::fs::read_to_string(&filepath)
        .unwrap()
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<Probe>, _>>()?;

    let mut global_map = probes[0].clone();
    let mut dist = vec![];

    while !probes.is_empty() {
        println!("Probes len: {}", probes.len());
        for idx in (0..probes.len()).rev() {
            if let Some(distance) = global_map.try_merge(&probes[idx]) {
                dist.push(distance);
                probes.swap_remove(idx);
            }
        }
    }

    println!("Part 1: {}", global_map.beacons.len());
    println!(
        "Part 2: {:?}",
        dist.iter()
            .tuple_combinations()
            .map(|((x1, y1, z1), (x2, y2, z2))| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
            .max()
    );
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19_step1() {
        assert!(true)
    }

    #[test]
    fn test_day19_step2() {
        assert!(true)
    }
}
