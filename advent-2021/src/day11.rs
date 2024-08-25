use shared::grid::{self, Grid, Position};

type Octopusses = Grid<usize>;

pub fn first(input: &str) -> String {
    let mut octopuses = Grid::from(input);
    (0..100)
        .map(|_| advance_one_step(&mut octopuses))
        .sum::<usize>()
        .to_string()
}

pub fn second(input: &str) -> String {
    let mut octopuses = Grid::from(&input);
    let number_of_octopuses = octopuses.height() * octopuses.width();
    (1..)
        .find(|_| advance_one_step(&mut octopuses) == number_of_octopuses)
        .expect("octopuses should flash simultaneously eventually")
        .to_string()
}

fn advance_one_step(octopuses: &mut Octopusses) -> usize {
    increase_energies(octopuses);
    let num_flashes = flash(octopuses);
    reset(octopuses);
    num_flashes
}

fn increase_energies(octopuses: &mut Octopusses) {
    for pos in octopuses
        .iter_row_major()
        .map(|(position, _)| position)
        .collect::<Vec<Position>>()
    {
        octopuses[pos] += 1;
    }
}

fn flashing_octopuses(octopuses: &Octopusses) -> Vec<Position> {
    octopuses
        .iter_row_major()
        .filter(|(_, &energy)| energy > 9)
        .map(|(pos, _)| pos)
        .collect()
}

fn flash(octopuses: &mut Octopusses) -> usize {
    let mut flashing_octopuses = flashing_octopuses(octopuses);
    let mut num_flashes = 0;
    while let Some(octopus) = flashing_octopuses.pop() {
        num_flashes += 1;
        for neighbor_octopus in grid::neighbors_including_diagonal(octopus) {
            octopuses[neighbor_octopus] += 1;
            if octopuses[neighbor_octopus] == 10 {
                flashing_octopuses.push(neighbor_octopus);
            }
        }
    }
    num_flashes
}

fn reset(octopuses: &mut Octopusses) {
    for flashing_octopus in flashing_octopuses(octopuses) {
        octopuses[flashing_octopus] = 0;
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 11;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1656);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1644);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 195);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 229);
    }
}
