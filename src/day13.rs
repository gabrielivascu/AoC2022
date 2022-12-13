use nom::{
    bytes::complete::tag, character::complete, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

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
    packets.push(parse_packet("[[2]]").unwrap().1);
    packets.push(parse_packet("[[6]]").unwrap().1);

    packets.sort_by(|lhs, rhs| match cmp_packets(lhs, rhs) {
        PacketOrder::Continue => std::cmp::Ordering::Equal,
        PacketOrder::Right => std::cmp::Ordering::Less,
        PacketOrder::Wrong => std::cmp::Ordering::Greater,
    });

    packets
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            if p.len() == 1 {
                match &p[0] {
                    PacketPart::List(l) => {
                        l.len() == 1
                            && (l[0] == PacketPart::Single(2) || l[0] == PacketPart::Single(6))
                    }
                    PacketPart::Single(_) => false,
                }
            } else {
                false
            }
        })
        .map(|(idx, _)| idx + 1)
        .product()
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

#[derive(Debug, PartialEq)]
enum PacketOrder {
    Right,
    Wrong,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
enum PacketPart {
    Single(i32),
    List(Vec<PacketPart>),
}

type Packet = Vec<PacketPart>;
type PacketPair = (Packet, Packet);

fn cmp_lists(lhs: &[PacketPart], rhs: &[PacketPart]) -> PacketOrder {
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
            (Some(a), Some(b)) => match cmp_parts(a, b) {
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

fn cmp_parts(lhs: &PacketPart, rhs: &PacketPart) -> PacketOrder {
    match (lhs, rhs) {
        (PacketPart::Single(a), PacketPart::Single(b)) => match a.cmp(b) {
            std::cmp::Ordering::Equal => PacketOrder::Continue,
            std::cmp::Ordering::Less => PacketOrder::Right,
            std::cmp::Ordering::Greater => PacketOrder::Wrong,
        },
        (PacketPart::List(a), PacketPart::List(b)) => cmp_lists(a, b),
        (PacketPart::Single(a), PacketPart::List(b)) => cmp_lists(&[PacketPart::Single(*a)], b),
        (PacketPart::List(a), PacketPart::Single(b)) => cmp_lists(a, &[PacketPart::Single(*b)]),
    }
}

fn cmp_packets(lhs: &Packet, rhs: &Packet) -> PacketOrder {
    cmp_parts(
        &PacketPart::List((*lhs).clone()),
        &PacketPart::List((*rhs).clone()),
    )
}

fn parse_integer(input: &str) -> IResult<&str, i32> {
    complete::i32(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    use nom::Parser;
    delimited(
        tag("["),
        separated_list0(
            tag(","),
            map(parse_integer, PacketPart::Single).or(map(parse_packet, PacketPart::List)),
        ),
        tag("]"),
    )(input)
}

fn build_packet_pairs(input: &str) -> Vec<PacketPair> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            (
                parse_packet(chunk[0]).unwrap().1,
                parse_packet(chunk[1]).unwrap().1,
            )
        })
        .collect()
}

fn build_single_packets(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|x| parse_packet(x).unwrap().1)
        .collect()
}
