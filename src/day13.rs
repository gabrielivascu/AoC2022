use serde::Deserialize;

pub fn solve_1(input: &str) -> usize {
    build_packet_pairs(input)
        .iter()
        .enumerate()
        .filter(|(_, pair)| cmp_packets(&pair.0, &pair.1) == PacketOrder::Right)
        .map(|(idx, _)| idx + 1)
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    let mut packets = build_single_packets(input);
    packets.push(serde_json::from_str("[[2]]").unwrap());
    packets.push(serde_json::from_str("[[6]]").unwrap());

    packets.sort_by(|lhs, rhs| match cmp_packets(lhs, rhs) {
        PacketOrder::Continue => std::cmp::Ordering::Equal,
        PacketOrder::Right => std::cmp::Ordering::Less,
        PacketOrder::Wrong => std::cmp::Ordering::Greater,
    });

    packets
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            ["[[2]]", "[[6]]"]
                .map(|x| serde_json::from_str(x).unwrap())
                .contains(*p)
        })
        .map(|(idx, _)| idx + 1)
        .product()
}

#[derive(Debug, PartialEq)]
enum PacketOrder {
    Right,
    Wrong,
    Continue,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
enum Packet {
    Single(i32),
    List(Vec<Packet>),
}

fn cmp_lists(lhs: &[Packet], rhs: &[Packet]) -> PacketOrder {
    let (mut lhs_iter, mut rhs_iter) = (lhs.iter(), rhs.iter());
    loop {
        match (lhs_iter.next(), rhs_iter.next()) {
            (None, Some(_)) => {
                return PacketOrder::Right;
            }
            (Some(_), None) => {
                return PacketOrder::Wrong;
            }
            (None, None) => {
                return PacketOrder::Continue;
            }
            (Some(lhs), Some(rhs)) => match cmp_packets(lhs, rhs) {
                PacketOrder::Right => {
                    return PacketOrder::Right;
                }
                PacketOrder::Wrong => {
                    return PacketOrder::Wrong;
                }
                PacketOrder::Continue => {}
            },
        }
    }
}

fn cmp_packets(lhs: &Packet, rhs: &Packet) -> PacketOrder {
    match (lhs, rhs) {
        (Packet::Single(lhs), Packet::Single(rhs)) => match lhs.cmp(rhs) {
            std::cmp::Ordering::Equal => PacketOrder::Continue,
            std::cmp::Ordering::Less => PacketOrder::Right,
            std::cmp::Ordering::Greater => PacketOrder::Wrong,
        },
        (Packet::List(lhs), Packet::List(rhs)) => cmp_lists(lhs, rhs),
        (Packet::Single(lhs), Packet::List(rhs)) => cmp_lists(&[Packet::Single(*lhs)], rhs),
        (Packet::List(lhs), Packet::Single(rhs)) => cmp_lists(lhs, &[Packet::Single(*rhs)]),
    }
}

fn build_packet_pairs(input: &str) -> Vec<(Packet, Packet)> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            (
                serde_json::from_str(chunk[0]).unwrap(),
                serde_json::from_str(chunk[1]).unwrap(),
            )
        })
        .collect()
}

fn build_single_packets(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|x| serde_json::from_str(x).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day13-sample.txt")), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day13-sample.txt")), 140);
    }
}
