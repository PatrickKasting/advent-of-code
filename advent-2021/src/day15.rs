use shared::{
    grid::{self, Grid, Position},
    search,
};

pub fn first(input: &str) -> String {
    let risk_levels = Grid::from(input);
    lowest_total_risk(&risk_levels, 1).to_string()
}

pub fn second(input: &str) -> String {
    let risk_levels = Grid::from(input);
    lowest_total_risk(&risk_levels, 5).to_string()
}

fn is_position_within_cave(
    risk_levels: &Grid<u8>,
    num_repetitions: usize,
    [row, column]: Position,
) -> bool {
    0 <= row
        && row < (risk_levels.height() * num_repetitions) as isize
        && 0 <= column
        && column < (risk_levels.width() * num_repetitions) as isize
}

fn risk(risk_levels: &Grid<u8>, [row, column]: Position) -> usize {
    let pos_within_tile = [
        row % risk_levels.height() as isize,
        column % risk_levels.width() as isize,
    ];
    let additional_risk =
        row as usize / risk_levels.height() + column as usize / risk_levels.width();
    (risk_levels[pos_within_tile] as usize + additional_risk - 1) % 9 + 1
}

fn lowest_total_risk(risk_levels: &Grid<u8>, num_repetitions: usize) -> usize {
    let start = [0, 0];
    let finish = [
        (risk_levels.height() * num_repetitions - 1) as isize,
        (risk_levels.width() * num_repetitions - 1) as isize,
    ];

    let successors = |position| {
        grid::neighbors(position)
            .into_iter()
            .filter(|&neighbor| is_position_within_cave(risk_levels, num_repetitions, neighbor))
            .map(|neighbor| (neighbor, risk(risk_levels, neighbor)))
    };
    search::cheapest_path_cost(start, successors, |position| position == finish)
        .expect("search should reach the bottom-right corner of the cave")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 15;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 40);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 652);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 315);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2938);
    }
}
