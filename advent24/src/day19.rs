use ahash::AHashMap;
use itertools::Itertools;

type Towel<'s> = &'s str;
type Design<'s> = &'s str;

pub fn first_answer(input: &str) -> String {
    let (towels, designs) = towels_and_designs(input);
    let mut cache = AHashMap::new();
    designs
        .filter(|design| number_of_different_ways(&mut cache, &towels, design) > 0)
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let (towels, designs) = towels_and_designs(input);
    let mut cache = AHashMap::new();
    designs
        .map(|design| number_of_different_ways(&mut cache, &towels, design))
        .sum::<usize>()
        .to_string()
}

fn number_of_different_ways<'design>(
    cache: &mut AHashMap<Design<'design>, usize>,
    towels: &[Towel],
    design: Design<'design>,
) -> usize {
    if let Some(&cached) = cache.get(design) {
        return cached;
    }
    let result = match design {
        "" => 1,
        _ => towels
            .iter()
            .map(|towel| match design.strip_prefix(towel) {
                Some(rest) => number_of_different_ways(cache, towels, rest),
                None => 0,
            })
            .sum(),
    };
    cache.insert(design, result);
    result
}

fn towels_and_designs(input: &'_ str) -> (Vec<Towel<'_>>, impl Iterator<Item = Design<'_>>) {
    let (towels, designs) = input
        .split_once("\n\n")
        .expect("towels and designs should be separated by an empty line");
    let towels = towels.split(", ").collect_vec();
    let designs = designs.lines();
    (towels, designs)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 19;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 6);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 369);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 16);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            761_826_581_538_190_usize,
        );
    }
}
