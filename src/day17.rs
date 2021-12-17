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
    fn new(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> Self {
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }
    fn is_in(&self, probe: &Probe) -> bool {
        self.xmin.le(&probe.x)
            && self.xmax.ge(&probe.x)
            && self.ymax.ge(&probe.y)
            && self.ymin.le(&probe.y)
    }
}

impl Probe {
    fn step(&self, target: &Target) -> Option<Self> {
        let (mut x, mut y, mut vx, mut vy, mut max_height) =
            (self.x, self.y, self.vx, self.vy, self.max_height);
        x += vx;
        y += vy;
        vx += if vx.eq(&0) {
            0
        } else if vx.gt(&0) {
            -1
        } else {
            1
        };
        vy -= 1;

        if y.gt(&max_height) {
            // If we reach an new top, update max height!
            max_height = y;
        }

        if vx.le(&0) && x.lt(&target.xmin) {
            // println!("Too low x:");
            // No hope to ever reach x_min
            return None;
        }

        if vx.ge(&0) && x.gt(&target.xmax) {
            // No hope to ever reach x_max
            // println!("Too high x");
            return None;
        }

        if vy.lt(&0) && y.lt(&target.ymin) {
            // We go deep but are already to low!
            return None;
        }

        // We can push a new point!
        Some(Self {
            x,
            y,
            vx,
            vy,
            max_height,
        })
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

    let mut h = 0;

    while let Some(new_probe) = probe.step(target) {
        h = new_probe.max_height;
        if target.is_in(&new_probe) {
            return Some(h);
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

    // let values = read_lines_to_vec_t("day_2021_17.data");
    let target_test = Target {
        xmin: 124,
        xmax: 174,
        ymin: -123,
        ymax: -86,
    };

    let mut pos = vec![];

    // Have to find better range here...
    for x_idx in -200..200 {
        for y_idx in -200..200 {
            pos.push((x_idx, y_idx));
        }
    }
    let part_1 = pos
        .iter()
        .filter_map(|(x, y)| max_height(*x, *y, &target_test))
        .max();

    let part_2 = pos
        .iter()
        .filter_map(|(x, y)| max_height(*x, *y, &target_test))
        .count();

    println!("{:?}", part_1);
    println!("{:?}", part_2);

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}
