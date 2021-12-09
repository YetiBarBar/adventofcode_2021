use adventofcode_2021::{utils::read_lines, Matrix2D};

#[must_use]
pub fn part_1(values: &Matrix2D<usize>) -> usize {
    (0..values.width)
        .flat_map(move |x| (0..values.height).map(move |y| (x, y)))
        .map(|(x, y)| {
            let val = values.get_x_y(x, y);
            if values.neighbour(x, y, false).iter().all(|v| v.gt(&val)) {
                val + 1
            } else {
                0
            }
        })
        .sum()
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_data: Vec<String> = read_lines("day_2021_9.data")?.map(Result::unwrap).collect();

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

    println!("{:?}", part_1(&matrix));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day9_part1() {
        let input_data = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        let input_data = input_data
            .lines()
            .map(|s| s.to_string())
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
        assert_eq!(part_1(&matrix), 15);
        //assert_eq!(part_2(&input), Some(168));
    }
}
