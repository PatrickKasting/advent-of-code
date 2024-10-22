use easy_cast::Cast;
use shared::grid::{neighbors_including_diagonal, Grid, Position};

type Octopusses = Grid<EnergyLevel>;
type EnergyLevel = usize;

const FLASHING_ENERGY_LEVEL: EnergyLevel = 10;

pub fn first_answer(input: &str) -> String {
    let mut octopusses = Octopusses::from(input);
    total_number_of_flashes(&mut octopusses, 100).to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut octopusses = Octopusses::from(input);
    number_of_steps_until_all_flash_simultaneously(&mut octopusses).to_string()
}

fn total_number_of_flashes(octopusses: &mut Octopusses, number_of_steps: usize) -> usize {
    let mut total_number_of_flashes = 0;
    for _ in 0..number_of_steps {
        total_number_of_flashes += number_of_flashes_during_step(octopusses);
    }
    total_number_of_flashes
}

fn number_of_steps_until_all_flash_simultaneously(octopusses: &mut Grid<usize>) -> usize {
    for number_of_steps in 1.. {
        number_of_flashes_during_step(octopusses);
        if are_all_zeroes(octopusses) {
            return number_of_steps;
        }
    }
    unreachable!("infinite loop iterator should never deplete");
}

fn number_of_flashes_during_step(octopusses: &mut Octopusses) -> usize {
    let mut number_of_flashes = 0;
    for row_index in 0..octopusses.height() {
        for column_index in 0..octopusses.width() {
            let position: Position = [row_index, column_index].cast();
            number_of_flashes += number_of_flashes_from_energy_increase(octopusses, position);
        }
    }
    zero_flashed_octopusses(octopusses);
    number_of_flashes
}

fn number_of_flashes_from_energy_increase(
    octopusses: &mut Octopusses,
    position: Position,
) -> usize {
    octopusses[position] += 1;
    if octopusses[position] == FLASHING_ENERGY_LEVEL {
        number_of_flashes_from_flash(octopusses, position)
    } else {
        0
    }
}

fn number_of_flashes_from_flash(octopusses: &mut Grid<usize>, position: [isize; 2]) -> usize {
    let mut number_of_flashes = 1;
    for neighbor in neighbors_including_diagonal(position) {
        if octopusses.get(neighbor).is_some() {
            number_of_flashes += number_of_flashes_from_energy_increase(octopusses, neighbor);
        }
    }
    number_of_flashes
}

fn zero_flashed_octopusses(octopusses: &mut Octopusses) {
    for row_index in 0..octopusses.height() {
        for column_index in 0..octopusses.width() {
            let energy_level = &mut octopusses[[row_index, column_index].cast()];
            if *energy_level >= FLASHING_ENERGY_LEVEL {
                *energy_level = 0;
            }
        }
    }
}

fn are_all_zeroes(octopusses: &Grid<usize>) -> bool {
    octopusses
        .iter_row_major()
        .all(|(_, &energy_level)| energy_level == 0)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 11;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1656);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1644);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 195);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 229);
    }
}
