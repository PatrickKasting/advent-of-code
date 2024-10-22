use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum LineStatus {
    IllegalCharacter(Character),
    CompletionString(Vec<Character>),
}

type Character = u8;
type Score = usize;

pub fn first_answer(input: &str) -> String {
    total_syntax_score(input).to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut completion_string_scores = completion_string_scores(input);
    completion_string_scores.sort_unstable();
    middle(&completion_string_scores).to_string()
}

fn total_syntax_score(input: &str) -> Score {
    let mut total_syntax_score = 0;
    for line_status in lines(input).map(line_status) {
        if let LineStatus::IllegalCharacter(illegal_character) = line_status {
            total_syntax_score += syntax_error_score(illegal_character);
        }
    }
    total_syntax_score
}

fn completion_string_scores(input: &str) -> Vec<usize> {
    let mut completion_string_scores = vec![];
    for line_status in lines(input).map(line_status) {
        if let LineStatus::CompletionString(completion_string) = line_status {
            if !completion_string.is_empty() {
                completion_string_scores.push(completion_string_score(&completion_string));
            }
        }
    }
    completion_string_scores
}

fn line_status(line: &[Character]) -> LineStatus {
    let mut unmatched_opening_characters = vec![];
    for &character in line {
        match character {
            b'(' | b'[' | b'{' | b'<' => unmatched_opening_characters.push(character),
            b')' | b']' | b'}' | b'>' => {
                let Some(opening_character) = unmatched_opening_characters.pop() else {
                    panic!("chunk should not contain more closing than opening characters")
                };
                if opening_character != opening_character_matching(character) {
                    return LineStatus::IllegalCharacter(character);
                }
            }
            _ => panic!("chunk should contain only opening and closing characters"),
        }
    }
    LineStatus::CompletionString(completion_string(unmatched_opening_characters))
}

fn opening_character_matching(closing_character: Character) -> Character {
    match closing_character {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => panic!(),
    }
}

fn syntax_error_score(closing_character: Character) -> Score {
    match closing_character {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!(),
    }
}

fn completion_string(unmatched_opening_characters: Vec<Character>) -> Vec<Character> {
    unmatched_opening_characters
        .into_iter()
        .rev()
        .map(closing_character_matching)
        .collect_vec()
}

fn closing_character_matching(opening_character: Character) -> Character {
    match opening_character {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => panic!("character should be '(', '[', '{{', or '>'"),
    }
}

fn completion_string_score(string: &[Character]) -> Score {
    let mut score = 0;
    for &character in string {
        score *= 5;
        score += completion_string_point_value(character);
    }
    score
}

fn completion_string_point_value(closing_character: Character) -> Score {
    match closing_character {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => panic!("character should be ')', ']', '}}', or '>'"),
    }
}

fn middle<T: Copy>(slice: &[T]) -> T {
    debug_assert!(slice.len() % 2 == 1, "length of slice should be odd");
    let index = slice.len() / 2;
    slice[index]
}

fn lines(input: &str) -> impl Iterator<Item = &[Character]> {
    input.lines().map(str::as_bytes)
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use crate::tests::test_on_input;

    use super::*;

    const DAY: usize = 10;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 26397);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 344_193);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 288_957);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 3_241_238_967_usize);
    }

    #[test]
    fn completion_string_score() {
        let cases: [(&[Character], _); 5] = [
            (b"}}]])})]", 288_957),
            (b")}>]})", 5566),
            (b"}}>}>))))", 1_480_781),
            (b"]]}}]}]}>", 995_444),
            (b"])}>", 294),
        ];
        test::cases(super::completion_string_score, cases);
    }
}
