const AT_LEAST_ONE_OPERAND: &str = "every operator should come with at least one operand";

type Value = usize;
type Type = usize;
type Version = usize;

pub fn first(input: &str) -> String {
    let bits = bits(input.trim());
    parse_packet(&bits).1.to_string()
}

pub fn second(input: &str) -> String {
    let bits = bits(input.trim());
    parse_packet(&bits).2.to_string()
}

fn add_bits(bits: &mut Vec<bool>, hexadecimal: char) {
    let digit = match hexadecimal {
        'A'..='F' => hexadecimal as u32 - 'A' as u32 + 10,
        _ => hexadecimal as u32 - '0' as u32,
    };
    for index in (0..4).rev() {
        bits.push((digit & (1 << index)) > 0);
    }
}

fn bits(hexadecimals: &str) -> Vec<bool> {
    let mut binary_data = Vec::new();
    for hexadecimal in hexadecimals.chars() {
        add_bits(&mut binary_data, hexadecimal)
    }
    binary_data
}

fn number(bits: &[bool]) -> Value {
    let mut number = 0;
    for (index, &bit) in bits.iter().rev().enumerate() {
        if bit {
            number += 1 << index;
        }
    }
    number
}

fn parse_header(bits: &[bool]) -> (&[bool], Version, Type) {
    let version = number(&bits[0..3]);
    let type_id = number(&bits[3..6]);
    (&bits[6..], version, type_id)
}

fn parse_literal_value(mut bits: &[bool]) -> (&[bool], Version, Value) {
    let mut value = 0;
    loop {
        value = (value << 4) + number(&bits[1..5]);
        if !bits[0] {
            break;
        }
        bits = &bits[5..];
    }
    (&bits[5..], 0, value)
}

fn parse_operands(mut bits: &[bool]) -> (&[bool], Version, Vec<Value>) {
    let mut versions = Vec::new();
    let mut operands = Vec::new();
    match bits[0] {
        true => {
            let num_sub_packages = number(&bits[1..12]);
            bits = &bits[12..];
            for _ in 0..num_sub_packages {
                let (remaining_bits, version, operand) = parse_packet(bits);
                bits = remaining_bits;
                versions.push(version);
                operands.push(operand);
            }
        }
        false => {
            let total_length = number(&bits[1..16]);
            let mut sub_bits = &bits[16..16 + total_length];
            while !sub_bits.is_empty() {
                let (remaining_bits, version, operand) = parse_packet(sub_bits);
                sub_bits = remaining_bits;
                versions.push(version);
                operands.push(operand);
            }
            bits = &bits[16 + total_length..];
        }
    }
    (bits, versions.into_iter().sum(), operands)
}

fn parse_operator(bits: &[bool], type_id: Type) -> (&[bool], Version, Value) {
    let (remaining_bits, version_sum, operands) = parse_operands(bits);
    let value = match type_id {
        0 => operands.into_iter().sum(),
        1 => operands.into_iter().product(),
        2 => operands.into_iter().min().expect(AT_LEAST_ONE_OPERAND),
        3 => operands.into_iter().max().expect(AT_LEAST_ONE_OPERAND),
        5 => {
            if operands[0] > operands[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if operands[0] < operands[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if operands[0] == operands[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("every packet type should be between zero and seven but not four"),
    };
    (remaining_bits, version_sum, value)
}

fn parse_packet(bits: &[bool]) -> (&[bool], Version, Value) {
    let (body, version, type_id) = parse_header(bits);
    let (remaining_bits, version_sum, value) = match type_id {
        4 => parse_literal_value(body),
        _ => parse_operator(body, type_id),
    };
    (remaining_bits, version + version_sum, value)
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 16;

    #[test]
    fn first_example() {
        let function = |input| parse_packet(&bits(input)).1;
        let cases = [
            ("D2FE28", 6),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1014);
    }

    #[test]
    fn second_example() {
        let function = |input| parse_packet(&bits(input)).2;
        let cases = [
            ("D2FE28", 2021),
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
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1922490999789usize);
    }
}
