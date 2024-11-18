use std::{cmp::Ordering, collections::VecDeque};

use ahash::AHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    One,
    Two,
}

type State = [Deck; 2];
type Deck = VecDeque<Card>;
type Card = usize;

pub fn first_answer(input: &str) -> String {
    let [mut deck1, mut deck2] = decks(input);
    let winning_deck = match combat(&mut deck1, &mut deck2) {
        Player::One => deck1,
        Player::Two => deck2,
    };
    score(&winning_deck).to_string()
}

pub fn second_answer(input: &str) -> String {
    let [mut deck1, mut deck2] = decks(input);
    let winning_deck = match recursive_combat(&mut deck1, &mut deck2) {
        Player::One => deck1,
        Player::Two => deck2,
    };
    score(&winning_deck).to_string()
}

fn combat(deck1: &mut Deck, deck2: &mut Deck) -> Player {
    loop {
        let [card1, card2] = match draw(deck1, deck2) {
            Ok(cards) => cards,
            Err(player) => return player,
        };
        let winner = higher_card_winner(card1, card2);
        place_on_bottom(deck1, deck2, card1, card2, winner);
    }
}

fn recursive_combat(deck1: &mut Deck, deck2: &mut Deck) -> Player {
    let mut previous_states: AHashSet<State> = AHashSet::new();
    loop {
        if !previous_states.insert([deck1.clone(), deck2.clone()]) {
            return Player::One;
        }

        let [card1, card2] = match draw(deck1, deck2) {
            Ok(cards) => cards,
            Err(player) => return player,
        };
        let winner = if card1 <= deck1.len() && card2 <= deck2.len() {
            let mut sub_deck1 = sub_deck(deck1, card1);
            let mut sub_deck2 = sub_deck(deck2, card2);
            recursive_combat(&mut sub_deck1, &mut sub_deck2)
        } else {
            higher_card_winner(card1, card2)
        };
        place_on_bottom(deck1, deck2, card1, card2, winner);
    }
}

fn draw(deck1: &mut Deck, deck2: &mut Deck) -> Result<[Card; 2], Player> {
    match [deck1.is_empty(), deck2.is_empty()] {
        [false, false] => Ok([deck1.pop_front(), deck2.pop_front()]
            .map(|card| card.expect("deck should not be empty due to pattern"))),
        [false, true] => Err(Player::One),
        [true, false] => Err(Player::Two),
        [true, true] => panic!("both cards should not be empty simultaneously"),
    }
}

fn sub_deck(deck: &Deck, number_of_cards: usize) -> Deck {
    deck.iter().copied().take(number_of_cards).collect()
}

fn higher_card_winner(card1: usize, card2: usize) -> Player {
    match card1.cmp(&card2) {
        Ordering::Less => Player::Two,
        Ordering::Greater => Player::One,
        Ordering::Equal => panic!("identical cards should not exist"),
    }
}

fn place_on_bottom(deck1: &mut Deck, deck2: &mut Deck, card1: Card, card2: Card, winner: Player) {
    match winner {
        Player::Two => {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
        Player::One => {
            deck1.push_back(card1);
            deck1.push_back(card2);
        }
    }
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .zip(1..)
        .map(|(card, multiplier)| card * multiplier)
        .sum()
}

fn decks(input: &str) -> [Deck; 2] {
    let (deck1, deck2) = input
        .split_once("\n\n")
        .expect("decks should be separated by a blank line");
    [deck(deck1), deck(deck2)]
}

fn deck(str: &str) -> Deck {
    str.lines()
        .skip(1)
        .map(|card| card.parse().expect("card should be numeric"))
        .collect()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 22;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 306);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 32629);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 291);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 32519);
    }
}
