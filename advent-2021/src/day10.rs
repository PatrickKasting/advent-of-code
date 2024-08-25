#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ParseResult {
    Incomplete(Vec<char>),
    Corrupted(char),
}

pub fn first(input: &str) -> String {
    line_scores(input, |parse_result| {
        if let ParseResult::Corrupted(closer) = parse_result {
            Some(corruption_score(closer))
        } else {
            None
        }
    })
    .into_iter()
    .sum::<usize>()
    .to_string()
}

pub fn second(input: &str) -> String {
    let mut scores = line_scores(input, |status| {
        if let ParseResult::Incomplete(unmatched_openers) = status {
            Some(autocomplete_score(&unmatched_openers))
        } else {
            None
        }
    });
    scores.sort_unstable();
    scores[scores.len() / 2].to_string()
}

fn line_scores(input: &str, line_score: impl Fn(ParseResult) -> Option<usize>) -> Vec<usize> {
    input
        .lines()
        .map(parse_line)
        .filter_map(line_score)
        .collect()
}

fn parse_line(line: &str) -> ParseResult {
    let mut unmatched_openers = Vec::new();
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => unmatched_openers.push(char),
            ')' | ']' | '}' | '>' => {
                if unmatched_openers
                    .last()
                    .is_some_and(|&opener| opener == matching_opener(char))
                {
                    unmatched_openers.pop();
                } else {
                    return ParseResult::Corrupted(char);
                }
            }
            _ => unreachable!("only brackets should occur in input"),
        }
    }
    debug_assert!(
        !unmatched_openers.is_empty(),
        "every line should be corrupted or incomplete"
    );
    ParseResult::Incomplete(unmatched_openers)
}

fn corruption_score(closer: char) -> usize {
    match closer {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("one of the four closing brackets should be used"),
    }
}

fn autocomplete_score(unmatched_openers: &[char]) -> usize {
    unmatched_openers
        .iter()
        .rev()
        .fold(0, |score, &opener| score * 5 + unmatched_score(opener))
}

fn unmatched_score(closer: char) -> usize {
    match closer {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!("one of the four closing brackets should be used"),
    }
}

fn matching_opener(closer: char) -> char {
    match closer {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("one of the four closing brackets should be used"),
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 10;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 26397);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 344193);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 288957);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 3241238967usize);
    }
}
