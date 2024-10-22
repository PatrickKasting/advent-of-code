use core::panic;
use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Packet {
    List(Vec<Packet>),
    Integer(Integer),
}

type Integer = u8;

pub fn first(input: &str) -> String {
    (1..)
        .zip(packet_pairs(input))
        .filter_map(|(index, [left, right])| compare(&left, &right).is_le().then_some(index))
        .sum::<usize>()
        .to_string()
}

pub fn second(input: &str) -> String {
    let packets = all_packets(input).collect_vec();
    divider_packet_indices(packets, ["[[2]]", "[[6]]"].map(packet))
        .into_iter()
        .product::<usize>()
        .to_string()
}

fn divider_packet_indices<const NUM_DIVIDER_PACKETS: usize>(
    mut packets: Vec<Packet>,
    divider_packets: [Packet; NUM_DIVIDER_PACKETS],
) -> [usize; NUM_DIVIDER_PACKETS] {
    packets.extend(divider_packets.clone());
    packets.sort_unstable_by(compare);
    divider_packets.map(|divider_packet| {
        packets
            .iter()
            .position(|packet| packet == &divider_packet)
            .expect("divider packet should be in list")
            + 1
    })
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    match [left, right] {
        [Packet::List(left), Packet::List(right)] => compare_lists(left, right),
        [Packet::List(left), Packet::Integer(right)] => {
            compare_lists(left, &[Packet::Integer(*right)])
        }
        [Packet::Integer(left), Packet::List(right)] => {
            compare_lists(&[Packet::Integer(*left)], right)
        }
        [Packet::Integer(left), Packet::Integer(right)] => left.cmp(right),
    }
}

fn compare_lists(left: &[Packet], right: &[Packet]) -> Ordering {
    let [mut left, mut right] = [left.iter(), right.iter()];
    loop {
        match [left.next(), right.next()] {
            [Some(left_element), Some(right_element)] => {
                let ordering = compare(left_element, right_element);
                if ordering.is_ne() {
                    return ordering;
                }
            }
            [Some(_), None] => return Ordering::Greater,
            [None, Some(_)] => return Ordering::Less,
            [None, None] => return Ordering::Equal,
        }
    }
}

fn packet_pairs(input: &str) -> impl Iterator<Item = [Packet; 2]> + '_ {
    input.split("\n\n").map(|pair| {
        pair.lines()
            .map(packet)
            .collect_vec()
            .try_into()
            .expect("packets should come in pairs")
    })
}

fn all_packets(input: &str) -> impl Iterator<Item = Packet> + '_ {
    input.lines().filter(|line| !line.is_empty()).map(packet)
}

fn packet(line: &str) -> Packet {
    let (remaining, packet) = parse_packet(line.as_bytes());
    debug_assert!(remaining.is_empty(), "entire line should be parsed");
    packet
}

fn parse_packet(bytes: &[u8]) -> (&[u8], Packet) {
    match bytes[0] {
        b'[' => {
            let (remaining, list) = parse_list(bytes);
            (remaining, Packet::List(list))
        }
        byte if byte.is_ascii_digit() => {
            let (remaining, integer) = parse_integer(bytes);
            (remaining, Packet::Integer(integer))
        }
        _ => panic!("packet shoud start with '[' or a digit"),
    }
}

fn parse_list(mut bytes: &[u8]) -> (&[u8], Vec<Packet>) {
    debug_assert_eq!(bytes[0], b'[', "first character of list should be '['");
    if bytes[1] == b']' {
        return (&bytes[2..], vec![]);
    }

    bytes = &bytes[1..];
    let mut elements = vec![];
    loop {
        let (remaining, packet) = parse_packet(bytes);
        bytes = remaining;
        elements.push(packet);
        if bytes[0] != b',' {
            break;
        }
        bytes = &bytes[1..];
    }
    debug_assert_eq!(bytes[0], b']', "last character of list should be ']'");
    (&bytes[1..], elements)
}

fn parse_integer(bytes: &[u8]) -> (&[u8], Integer) {
    let (number_of_digits, integer) = bytes
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .fold((0, 0), |(index, integer), byte| {
            (index + 1, integer * 10 + (byte - b'0'))
        });
    (&bytes[number_of_digits..], integer)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 13;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 13);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4821);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 140);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 21890);
    }
}
