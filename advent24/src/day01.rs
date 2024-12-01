use itertools::Itertools;
use shared::string::usizes;

type List = Vec<LocationId>;
type LocationId = usize;

pub fn first_answer(input: &str) -> String {
    let [left_list, right_list] = lists(input);
    sum_of_distances(left_list, right_list).to_string()
}

pub fn second_answer(input: &str) -> String {
    let [left_list, right_list] = lists(input);
    similarity_score(left_list, right_list).to_string()
}

fn sum_of_distances(mut left_list: List, mut right_list: List) -> LocationId {
    left_list.sort_unstable();
    right_list.sort_unstable();
    left_list
        .into_iter()
        .zip_eq(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn similarity_score(left_list: List, right_list: List) -> LocationId {
    let counts = right_list.into_iter().counts();
    left_list
        .into_iter()
        .map(|location_id| location_id * counts.get(&location_id).copied().unwrap_or_default())
        .sum()
}

fn lists(input: &str) -> [List; 2] {
    let localtion_ids = usizes(input);
    [0, 1].map(|start_index| {
        (start_index..localtion_ids.len())
            .step_by(2)
            .map(|index| localtion_ids[index])
            .collect_vec()
    })
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 1;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 11);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_530_215);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 31);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 26_800_609);
    }
}
