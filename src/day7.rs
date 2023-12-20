use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn card(j: Card, char: char) -> Card {
    match char {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => j,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => unreachable!("card should be one of the 13 known ones"),
    }
}

type Hand = [Card; 5];
type Bid = usize;
type HandType = Vec<usize>;

fn hand(j: Card, hand: &str) -> Hand {
    hand.chars()
        .map(|char| card(j, char))
        .collect_vec()
        .try_into()
        .expect("a hand should contain five cards")
}

fn player(j: Card, line: &str) -> (Hand, Bid) {
    let (hand, bid) = line.split_once(' ').expect("a line should include a space");
    (
        self::hand(j, hand),
        bid.parse().expect("bid should be numeric"),
    )
}

fn hand_type(mut hand: Hand) -> HandType {
    hand.sort();
    let mut group_sizes = hand
        .group_by(|&left, &right| left == right)
        .filter_map(|group| (group[0] != Card::Joker).then_some(group.len()))
        .collect_vec();
    group_sizes.sort_unstable_by_key(|&group_size| usize::MAX - group_size);

    if group_sizes.is_empty() {
        group_sizes.push(0);
    }
    for _ in hand.into_iter().take_while(|&card| card == Card::Joker) {
        group_sizes[0] += 1;
    }

    group_sizes
}

fn total_winnings(j: Card, input: &str) -> usize {
    let mut strengths_and_bids = input
        .lines()
        .map(|line| player(j, line))
        .map(|(hand, bid)| ((hand_type(hand), hand), bid))
        .collect_vec();
    strengths_and_bids.sort_by_key(|(strength, _)| strength.clone());

    (1..)
        .zip(strengths_and_bids)
        .map(|(rank, (_, bid))| rank * bid)
        .sum()
}

pub fn first(input: String) -> String {
    total_winnings(Card::Jack, &input).to_string()
}

pub fn second(input: String) -> String {
    total_winnings(Card::Joker, &input).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 7;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 6440);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 250957639);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 5905);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 251515496);
    }
}
