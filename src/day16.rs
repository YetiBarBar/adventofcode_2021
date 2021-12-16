use std::path::PathBuf;

use adventofcode_tooling::AocError;

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    type_id: usize,
    message: Message,
}

impl Packet {
    #[must_use]
    fn version_sum(&self) -> usize {
        match &self.message {
            Message::Literal(_) => self.version,
            Message::Operator(v) => self.version + v.iter().map(|p| p.version_sum()).sum::<usize>(),
        }
    }

    #[must_use]
    fn evaluate(&self) -> usize {
        match &self.message {
            Message::Literal(val) => {
                if self.type_id == 4 {
                    *val
                } else {
                    panic!("Unexpected type 4 not literal!");
                }
            }
            Message::Operator(v) => match &self.type_id {
                0 => v.iter().map(|op| op.evaluate()).sum(),
                1 => v.iter().map(|op| op.evaluate()).product(),
                2 => v.iter().map(|op| op.evaluate()).min().unwrap(),
                3 => v.iter().map(|op| op.evaluate()).max().unwrap(),
                5 => {
                    let values: Vec<_> = v.iter().map(|op| op.evaluate()).collect();
                    if values[0].gt(&values[1]) {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let values: Vec<_> = v.iter().map(|op| op.evaluate()).collect();
                    if values[0].lt(&values[1]) {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let values: Vec<_> = v.iter().map(|op| op.evaluate()).collect();
                    if values[0].eq(&values[1]) {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum Message {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[must_use]
fn n_to_m_bytes_to_usize(bits: &[usize], n: usize, m: usize) -> Option<usize> {
    if m.lt(&n) {
        None
    } else {
        Some(
            bits.iter()
                .skip(n)
                .take(m - n)
                .rev()
                .enumerate()
                .map(|(pos, bit)| *bit << pos)
                .sum::<usize>(),
        )
    }
}

fn parse_input(data: &[usize]) -> Option<(Packet, usize)> {
    let version = n_to_m_bytes_to_usize(data, 0, 3)?;
    let packet_id = n_to_m_bytes_to_usize(data, 3, 6)?;

    // First case: A literal
    if packet_id == 4 {
        // We have a literal value
        let mut values = data[6..]
            .chunks(5)
            .take_while(|chunk| chunk[0] != 0)
            .flat_map(|chunk| chunk[1..].iter())
            .copied()
            .collect::<Vec<usize>>();
        values.extend(
            data[6..].chunks(5).skip(values.len() / 4).next().unwrap()[1..]
                .iter()
                .copied(),
        );

        let literal = n_to_m_bytes_to_usize(&values, 0, values.len())?;
        return Some((
            Packet {
                version,
                type_id: packet_id,
                message: Message::Literal(literal),
            },
            6 + 5 * values.len() / 4,
        ));
    }

    // Second case: a unique subpacket
    if data[6] == 0 {
        let sub_packet_len = n_to_m_bytes_to_usize(&data[7..7 + 15], 0, 15)?;
        let mut parsed_size = 0;
        let mut msg = Vec::new();
        loop {
            let (sub_packet, len) = parse_input(&data[(15 + 7 + parsed_size)..])?;

            msg.push(sub_packet);
            parsed_size += len;
            if parsed_size.ge(&sub_packet_len) {
                break;
            }
        }
        return Some((
            Packet {
                version,
                type_id: packet_id,
                message: Message::Operator(msg),
            },
            7 + 15 + sub_packet_len,
        ));
    }

    if data[6] == 1 {
        let sub_packet_count = n_to_m_bytes_to_usize(&data[7..7 + 11], 0, 11)?;
        let mut parsed_size = 0;
        let mut msg = Vec::new();
        for _ in 0..sub_packet_count {
            let (sub_packet, len) = parse_input(&data[(11 + 7 + parsed_size)..])?;
            msg.push(sub_packet);
            parsed_size += len;
        }
        return Some((
            Packet {
                version,
                type_id: packet_id,
                message: Message::Operator(msg),
            },
            7 + 11 + parsed_size,
        ));
    }

    panic!("Unparsable message")
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_1(data: &[usize]) -> Option<usize> {
    let operations = parse_input(data);
    Some(operations?.0.version_sum())
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_2(data: &[usize]) -> Option<usize> {
    let operations = parse_input(data)?;
    Some(operations.0.evaluate())
}

/// Process solutions for day 16
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2021_16.data");

    let input_data = std::fs::read_to_string(filepath).unwrap();
    let input_data = input_data
        .chars()
        .filter_map(|ch| match ch {
            '0' => Some(vec![0, 0, 0, 0]),
            '1' => Some(vec![0, 0, 0, 1]),
            '2' => Some(vec![0, 0, 1, 0]),
            '3' => Some(vec![0, 0, 1, 1]),
            '4' => Some(vec![0, 1, 0, 0]),
            '5' => Some(vec![0, 1, 0, 1]),
            '6' => Some(vec![0, 1, 1, 0]),
            '7' => Some(vec![0, 1, 1, 1]),
            '8' => Some(vec![1, 0, 0, 0]),
            '9' => Some(vec![1, 0, 0, 1]),
            'A' => Some(vec![1, 0, 1, 0]),
            'B' => Some(vec![1, 0, 1, 1]),
            'C' => Some(vec![1, 1, 0, 0]),
            'D' => Some(vec![1, 1, 0, 1]),
            'E' => Some(vec![1, 1, 1, 0]),
            'F' => Some(vec![1, 1, 1, 1]),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", part_1(&input_data));
    println!("Part 2: {:?}", part_2(&input_data));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_step1() {
        let input_data = "8A004A801A8002F478";
        let input_data = input_data
            .chars()
            .filter_map(|ch| match ch {
                '0' => Some(vec![0, 0, 0, 0]),
                '1' => Some(vec![0, 0, 0, 1]),
                '2' => Some(vec![0, 0, 1, 0]),
                '3' => Some(vec![0, 0, 1, 1]),
                '4' => Some(vec![0, 1, 0, 0]),
                '5' => Some(vec![0, 1, 0, 1]),
                '6' => Some(vec![0, 1, 1, 0]),
                '7' => Some(vec![0, 1, 1, 1]),
                '8' => Some(vec![1, 0, 0, 0]),
                '9' => Some(vec![1, 0, 0, 1]),
                'A' => Some(vec![1, 0, 1, 0]),
                'B' => Some(vec![1, 0, 1, 1]),
                'C' => Some(vec![1, 1, 0, 0]),
                'D' => Some(vec![1, 1, 0, 1]),
                'E' => Some(vec![1, 1, 1, 0]),
                'F' => Some(vec![1, 1, 1, 1]),
                _ => None,
            })
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(part_1(&input_data), Some(16));
    }
    #[test]
    fn test_day16_step2() {
        let inputs = [
            "C200B40A82",
            "04005AC33890",
            "880086C3E88112",
            "CE00C43D881120",
            "D8005AC2A8F0",
            "F600BC2D8F",
            "9C005AC2F8F0",
            "9C0141080250320F1802104A08",
        ];
        let expected = [
            Some(3),
            Some(54),
            Some(7),
            Some(9),
            Some(1),
            Some(0),
            Some(0),
            Some(1),
        ];

        let mut computed = vec![];
        for input_data in inputs {
            let input_data = input_data
                .chars()
                .filter_map(|ch| match ch {
                    '0' => Some(vec![0, 0, 0, 0]),
                    '1' => Some(vec![0, 0, 0, 1]),
                    '2' => Some(vec![0, 0, 1, 0]),
                    '3' => Some(vec![0, 0, 1, 1]),
                    '4' => Some(vec![0, 1, 0, 0]),
                    '5' => Some(vec![0, 1, 0, 1]),
                    '6' => Some(vec![0, 1, 1, 0]),
                    '7' => Some(vec![0, 1, 1, 1]),
                    '8' => Some(vec![1, 0, 0, 0]),
                    '9' => Some(vec![1, 0, 0, 1]),
                    'A' => Some(vec![1, 0, 1, 0]),
                    'B' => Some(vec![1, 0, 1, 1]),
                    'C' => Some(vec![1, 1, 0, 0]),
                    'D' => Some(vec![1, 1, 0, 1]),
                    'E' => Some(vec![1, 1, 1, 0]),
                    'F' => Some(vec![1, 1, 1, 1]),
                    _ => None,
                })
                .flatten()
                .collect::<Vec<_>>();
            computed.push(part_2(&input_data));
        }
        assert_eq!(expected.to_vec(), computed);
    }
}
