use std::{collections::BTreeSet, ops::RangeInclusive};

use ahash::AHashMap;
use itertools::Itertools;
use shared::{search::bijections, string::usizes};

type Rules<'input> = AHashMap<Field<'input>, Vec<RangeInclusive<Value>>>;
type Field<'input> = &'input str;
type Value = usize;
type Ticket = Vec<Value>;
type Position = usize;

pub fn first_answer(input: &str) -> String {
    let (rules, _, nearby_tickets) = notes(input);
    nearby_tickets
        .flat_map(|ticket| invalid_values(&rules, &ticket))
        .sum::<Value>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let (rules, my_ticket, nearby_tickets) = notes(input);
    let valid_nearby_tickets = nearby_tickets
        .filter(|ticket| invalid_values(&rules, ticket).is_empty())
        .collect_vec();
    field_order(&rules, &valid_nearby_tickets)
        .into_iter()
        .filter(|(_, field)| field.starts_with("departure"))
        .map(|(position, _)| my_ticket[position])
        .product::<Value>()
        .to_string()
}

fn invalid_values<'input>(rules: &'input Rules, ticket: &'input Ticket) -> Vec<Value> {
    ticket
        .iter()
        .copied()
        .filter(|value| !rules.values().flatten().any(|range| range.contains(value)))
        .collect_vec()
}

fn field_order<'input>(
    rules: &'input Rules,
    valid_nearby_tickets: &'input [Ticket],
) -> AHashMap<Position, Field<'input>> {
    let possible_fields = (0..valid_nearby_tickets[0].len())
        .map(|position| {
            let possibilities = possible_fields(rules, valid_nearby_tickets, position);
            (position, possibilities)
        })
        .collect();
    bijections(possible_fields)
        .into_iter()
        .exactly_one()
        .expect("only one field order should be possible")
}

fn possible_fields<'input>(
    rules: &'input Rules,
    tickets: &'input [Ticket],
    position: Position,
) -> BTreeSet<Field<'input>> {
    let values_at_position = values_at_position(tickets, position);
    rules
        .iter()
        .filter(|(_, ranges)| valid_values(ranges, &values_at_position))
        .map(|(&field, _)| field)
        .collect()
}

fn values_at_position(tickets: &[Ticket], position: Position) -> Vec<Value> {
    tickets.iter().map(|ticket| ticket[position]).collect_vec()
}

fn valid_values(ranges: &[RangeInclusive<Value>], values: &[Value]) -> bool {
    values
        .iter()
        .all(|value| ranges.iter().any(|range| range.contains(value)))
}

fn notes(input: &str) -> (Rules, Ticket, impl Iterator<Item = Ticket> + '_) {
    let [rules, my_ticket, nearby_tickets]: [&str; 3] = input
        .split("\n\n")
        .collect_vec()
        .try_into()
        .expect("input should contain three sections separated by empty lines");
    let nearby_tickets = nearby_tickets.lines().skip(1).map(usizes);
    (self::rules(rules), usizes(my_ticket), nearby_tickets)
}

fn rules(str: &str) -> Rules {
    str.lines().map(rule).collect()
}

fn rule(line: &str) -> (&str, Vec<RangeInclusive<Value>>) {
    let (name, ranges) = line
        .split_once(": ")
        .expect("name and ranges should be separated by a colon");
    let ranges = usizes(ranges)
        .chunks(2)
        .map(|pair| pair[0]..=pair[1])
        .collect_vec();
    (name, ranges)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 16;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 71);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 29759);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            1_307_550_234_719_usize,
        );
    }
}
