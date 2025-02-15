use std::cmp::Ordering;
use std::collections::BinaryHeap;

use adventofcode_2021::Matrix2D;
use adventofcode_tooling::{read_lines, AocError};

#[must_use]
fn produce_big_matrix(
    input: &Matrix2D<usize>,
    times: usize,
    compute_func: impl Fn(usize, usize) -> usize,
) -> Matrix2D<usize> {
    let rows = input.rows();
    let compute_func = &compute_func;

    let values = (0..times)
        .flat_map(|idx| {
            rows.iter()
                .flat_map(|v| {
                    (0..times).flat_map(|idx| v.iter().map(move |&value| compute_func(value, idx)))
                })
                .map(move |value| compute_func(value, idx))
        })
        .collect::<Vec<_>>();

    Matrix2D {
        width: input.width * 5,
        height: input.height * 5,
        values,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(
    data: &Matrix2D<usize>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = Matrix2D {
        width: data.width,
        height: data.height,
        values: vec![usize::MAX; data.values.len()],
    };

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.values[start.1 * dist.width + start.0] = 0;

    heap.push(State {
        cost: 0,
        x: start.0,
        y: start.1,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, x, y }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if (x, y) == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist.values[y * dist.width + x] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in data.get_neighbours_coord(x, y, false) {
            let next = State {
                cost: cost + data.get_x_y(edge.0, edge.1),
                x: edge.0,
                y: edge.1,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist.values[next.x + next.y * data.height] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.values[next.x + next.y * data.height] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_1(data: &Matrix2D<usize>) -> Option<usize> {
    shortest_path(data, (0, 0), (data.width - 1, data.height - 1))
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_2(data: &Matrix2D<usize>) -> Option<usize> {
    let data = produce_big_matrix(data, 5, |value: usize, index: usize| {
        if (value + index).le(&9) {
            value + index
        } else {
            (value + index) % 10 + 1
        }
    });
    shortest_path(&data, (0, 0), (data.width - 1, data.height - 1))
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    let input_data: Vec<String> = read_lines("day_2021_15.data")?
        .map(Result::unwrap)
        .collect();

    let (width, height) = (input_data.first().unwrap().len(), input_data.len());
    let matrix = Matrix2D {
        width,
        height,
        values: input_data
            .iter()
            .flat_map(|s| s.chars())
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect(),
    };
    println!("Part 1: {:?}", part_1(&matrix));
    println!("Part 1: {:?}", part_2(&matrix));

    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_step1() {
        let input = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let input_data = input
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();

        let (width, height) = (input_data.first().unwrap().len(), input_data.len());
        let matrix = Matrix2D {
            width,
            height,
            values: input_data
                .iter()
                .flat_map(|s| s.chars())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        };
        assert_eq!(part_1(&matrix), Some(40));
    }
    #[test]
    fn test_day15_part2() {
        let input = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let input_data = input
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();

        let (width, height) = (input_data.first().unwrap().len(), input_data.len());
        let matrix = Matrix2D {
            width,
            height,
            values: input_data
                .iter()
                .flat_map(|s| s.chars())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        };
        assert_eq!(part_2(&matrix), Some(315));
    }
}
