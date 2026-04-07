use ahash::AHashMap;
use itertools::Itertools;

type Towel<'s> = &'s str;
type Design<'s> = &'s str;

pub fn first_answer(input: &str) -> String {
    let (towels, designs) = towels_and_designs(input);
    let mut cache = AHashMap::new();
    designs
        .filter(|design| is_possible(&mut cache, &towels, design))
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn is_possible<'design>(
    cache: &mut AHashMap<Design<'design>, bool>,
    towels: &[Towel],
    design: Design<'design>,
) -> bool {
    if let Some(&cached) = cache.get(design) {
        return cached;
    }
    let result = design.is_empty()
        || towels.iter().any(|towel| {
            design
                .strip_prefix(towel)
                .is_some_and(|rest| is_possible(cache, towels, rest))
        });
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

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }
}
