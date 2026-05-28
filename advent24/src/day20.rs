use ahash::AHashMap;
use shared::{
    grid::{self, Direction, Grid, Position},
    search,
    vector::Vector,
};

type Racetrack = Grid<u8>;
type Cheat = [Position; 2];
type Picosecond = usize;

pub fn first_answer(input: &str) -> String {
    let racetrack = Racetrack::from(input);
    times_saved(&racetrack)
        .filter(|&saved| saved >= 100)
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn times_saved(racetrack: &Racetrack) -> impl Iterator<Item = Picosecond> {
    let distances = distances(racetrack);
    all_cheats(racetrack).filter_map(move |cheat| time_saved(&distances, cheat))
}

fn distances(racetrack: &Racetrack) -> AHashMap<Position, usize> {
    let successors = |position| {
        grid::orthogonal_neighbors(position)
            .into_iter()
            .filter(|&neighbor| racetrack[neighbor] != b'#')
    };
    search::distances(source(racetrack), successors)
}

fn source(racetrack: &Racetrack) -> Position {
    racetrack
        .find(|_, &element| element == b'S')
        .expect("racetrack should have a start")
        .0
}

// fn target(racetrack: &Racetrack) -> Position {
//     racetrack
//         .find(|_, &element| element == b'E')
//         .expect("racetrack should have an end")
//         .0
// }

fn all_cheats(racetrack: &Racetrack) -> impl Iterator<Item = Cheat> {
    let mut all_cheats = vec![];
    all_cheats.extend(cheats(racetrack, [0, 1]));
    all_cheats.extend(cheats(racetrack, [1, 0]));
    all_cheats
        .into_iter()
        .flat_map(|[from, to]| [[from, to], [to, from]])
}

fn cheats(racetrack: &Racetrack, direction: Direction) -> impl Iterator<Item = Cheat> {
    racetrack
        .iter_row_major()
        .filter_map(move |(position, &element)| {
            if element != b'#' {
                return None;
            }

            let cheat = [-1, 1].map(|orientation| position.add(direction.mul(orientation)));
            let is_possible = cheat.into_iter().all(|neighbor| {
                racetrack
                    .get(neighbor)
                    .is_some_and(|&element| element != b'#')
            });
            is_possible.then_some(cheat)
        })
}

fn time_saved(distances: &AHashMap<Position, usize>, [from, to]: Cheat) -> Option<Picosecond> {
    distances[&to]
        .checked_sub(distances[&from])
        .and_then(|saved| saved.checked_sub(2))
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle, test};
    use itertools::Itertools;

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 20;

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1338);
    }

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }

    #[test]
    fn times_saved() {
        let racetrack = Racetrack::from(input(DAY, Input::Example(0)));
        let counts = super::times_saved(&racetrack).counts();
        assert_eq!(counts.len(), 11);

        let cases = [
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ];
        test::cases(|saved| counts[&saved], cases);
    }
}
