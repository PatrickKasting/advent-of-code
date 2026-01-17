use std::ops::RangeInclusive;

use ahash::AHashMap;

type Validation = fn(Value) -> bool;
type Passport<'input> = AHashMap<Key<'input>, Value<'input>>;
type Key<'input> = &'input str;
type Value<'input> = &'input str;

pub fn first_answer(input: &str) -> String {
    passports(input)
        .filter(are_required_keys_present)
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    passports(input)
        .filter(are_fields_valid)
        .count()
        .to_string()
}

fn are_required_keys_present(passport: &Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .all(|key| passport.contains_key(key))
}

fn are_fields_valid(passport: &Passport) -> bool {
    let validations: [(Key, Validation); 7] = [
        ("byr", is_birth_year_valid),
        ("iyr", is_issue_year_valid),
        ("eyr", is_expiration_year_valid),
        ("hgt", is_height_valid),
        ("hcl", is_hair_color_valid),
        ("ecl", is_eye_color_valid),
        ("pid", is_passport_id_valid),
    ];
    validations
        .into_iter()
        .all(|(key, validation)| passport.get(key).is_some_and(|value| validation(value)))
}

fn is_birth_year_valid(value: Value) -> bool {
    value.len() == 4 && is_number_within(1920..=2002, 10, value)
}

fn is_issue_year_valid(value: Value) -> bool {
    value.len() == 4 && is_number_within(2010..=2020, 10, value)
}

fn is_expiration_year_valid(value: Value) -> bool {
    value.len() == 4 && is_number_within(2020..=2030, 10, value)
}

fn is_height_valid(value: Value) -> bool {
    if let Some(value) = value.strip_suffix("cm") {
        is_number_within(150..=193, 10, value)
    } else if let Some(value) = value.strip_suffix("in") {
        is_number_within(59..=76, 10, value)
    } else {
        false
    }
}

fn is_hair_color_valid(value: Value) -> bool {
    #[expect(
        clippy::unreadable_literal,
        reason = "underscore spilts color hex oddly"
    )]
    if let Some(value) = value.strip_prefix('#') {
        value.len() == 6 && is_number_within(0x000000..=0xffffff, 16, value)
    } else {
        false
    }
}

fn is_eye_color_valid(value: Value) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
}

fn is_passport_id_valid(value: Value) -> bool {
    value.len() == 9 && value.chars().all(|char| char.is_ascii_digit())
}

fn is_number_within(range: RangeInclusive<usize>, radix: u32, value: Value) -> bool {
    usize::from_str_radix(value, radix).is_ok_and(|number| range.contains(&number))
}

fn passports(input: &str) -> impl Iterator<Item = Passport<'_>> {
    input.split("\n\n").map(passport)
}

fn passport(str: &str) -> Passport<'_> {
    str.split_whitespace().map(field).collect()
}

fn field(str: &str) -> (Key<'_>, Value<'_>) {
    str.split_once(':')
        .expect("key and value should ne separated by a colon")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 4;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 2);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 264);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 4);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 224);
    }
}
