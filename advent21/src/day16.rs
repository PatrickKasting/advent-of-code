use core::str;
use std::{
    cmp,
    ops::{Add, Mul},
};

use bitvec::{field::BitField, order::Msb0, slice::BitSlice, vec::BitVec, view::BitView};

type Packet = BitVec<usize, BitOrder>;
type Bits<'bits> = &'bits BitSlice<usize, BitOrder>;
type BitOrder = Msb0;
type VersionNumber = Decimal;
type Value = Decimal;
type Decimal = usize;

pub fn first_answer(input: &str) -> String {
    let packet = packet(input);
    let (_, version_number_sum, _) = evaluation(&packet);
    version_number_sum.to_string()
}

pub fn second_answer(input: &str) -> String {
    let packet = packet(input);
    let (_, _, value) = evaluation(&packet);
    value.to_string()
}

fn evaluation(packet: Bits) -> (Bits, VersionNumber, Value) {
    match type_id(packet) {
        0 => operation(packet, <Decimal as Add>::add),
        1 => operation(packet, <Decimal as Mul>::mul),
        2 => operation(packet, cmp::min),
        3 => operation(packet, cmp::max),
        4 => literal(packet),
        5 => operation(packet, |left, right| (left > right).into()),
        6 => operation(packet, |left, right| (left < right).into()),
        7 => operation(packet, |left, right| (left == right).into()),
        _ => panic!("type id should be between zero and seven"),
    }
}

fn operation(
    packet: Bits,
    operator: impl Fn(Value, Value) -> Value,
) -> (Bits, VersionNumber, Value) {
    let length_type_id = packet[6];
    let (remaining, version_number_sum, sub_values) = if length_type_id {
        sub_evaluations_number_of_sub_packets(packet)
    } else {
        sub_evaluations_total_length(packet)
    };

    let value = sub_values
        .into_iter()
        .reduce(operator)
        .expect("number of sub-packets should be at least one");
    (
        remaining,
        version_number(packet) + version_number_sum,
        value,
    )
}

fn sub_evaluations_total_length(packet: Bits) -> (Bits, VersionNumber, Vec<Value>) {
    let total_number_of_bits = decimal(&packet[7..22]);
    let mut sub_packets = &packet[22..22 + total_number_of_bits];
    let mut version_number_sum = 0;
    let mut values = vec![];
    while !sub_packets.is_empty() {
        let (rest, sub_version_number_sum, value) = evaluation(sub_packets);
        sub_packets = rest;
        version_number_sum += sub_version_number_sum;
        values.push(value);
    }
    (
        &packet[22 + total_number_of_bits..],
        version_number_sum,
        values,
    )
}

fn sub_evaluations_number_of_sub_packets(packet: Bits) -> (Bits, VersionNumber, Vec<Value>) {
    let number_of_sub_packets = decimal(&packet[7..18]);
    let mut remaining = &packet[18..];
    let mut version_number_sum = 0;
    let mut values = vec![];
    for _ in 0..number_of_sub_packets {
        let (rest, sub_version_number, value) = evaluation(remaining);
        remaining = rest;
        version_number_sum += sub_version_number;
        values.push(value);
    }
    (remaining, version_number_sum, values)
}

fn literal(packet: Bits) -> (Bits, VersionNumber, Value) {
    let mut remaining = &packet[6..];
    let mut value_bits: BitVec<usize, BitOrder> = BitVec::new();
    while remaining[0] {
        value_bits.extend_from_bitslice(&remaining[1..5]);
        remaining = &remaining[5..];
    }
    value_bits.extend_from_bitslice(&remaining[1..5]);
    remaining = &remaining[5..];
    (remaining, version_number(packet), decimal(&value_bits))
}

fn version_number(packet: Bits) -> VersionNumber {
    decimal(&packet[0..3])
}

fn type_id(packet: Bits) -> Decimal {
    decimal(&packet[3..6])
}

fn decimal(bits: Bits) -> Decimal {
    bits.load_be()
}

fn packet(mut input: &str) -> Packet {
    input = input.trim_end();

    let mut packet = Packet::new();
    for index in 0..input.len() {
        let byte =
            u8::from_str_radix(&input[index..=index], 16).expect("hexidecimal digit should parse");
        packet.extend_from_bitslice(&byte.view_bits::<BitOrder>()[4..]);
    }
    packet
}

#[cfg(test)]
mod tests {
    use bitvec::bits;
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 16;

    #[test]
    fn first_answer_example() {
        let function = |input| evaluation(&packet(input)).1;
        let cases = [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1014);
    }

    #[test]
    fn second_examples() {
        let function = |input| evaluation(&packet(input)).2;
        let cases = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            1_922_490_999_789_usize,
        );
    }

    #[test]
    fn literal() {
        let input = small_example_packet();
        let actual = super::literal(&input);
        let expected = (bits![usize, BitOrder; 0, 0, 0], 6, 2021);
        assert_eq!(actual, expected);
    }

    #[test]
    fn version_number() {
        let actual = super::version_number(&small_example_packet());
        let expected = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    fn type_id() {
        let actual = super::type_id(&small_example_packet());
        let expected = 4;
        assert_eq!(actual, expected);
    }

    fn small_example_packet() -> Packet {
        packet("D2FE28")
    }
}
