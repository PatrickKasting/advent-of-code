use itertools::Itertools;
use shared::{
    grid::{self, Direction, Grid, Position},
    vector::Vector,
};

type SeatLayout = Grid<char>;

pub fn first_answer(input: &str) -> String {
    let mut seat_layout = SeatLayout::from(input);
    loop {
        let next = next_considering_adjacent_seats(&seat_layout);
        if next == seat_layout {
            return number_of_occupied_seats(&next).to_string();
        }
        seat_layout = next;
    }
}

pub fn second_answer(input: &str) -> String {
    let mut seat_layout = SeatLayout::from(input);
    let visible_seats = visible_seats_from_all(&seat_layout);
    loop {
        let next = next_considering_visible_seats(&seat_layout, &visible_seats);
        if next == seat_layout {
            return number_of_occupied_seats(&next).to_string();
        }
        seat_layout = next;
    }
}

fn next_considering_adjacent_seats(seat_layout: &SeatLayout) -> SeatLayout {
    seat_layout.map(|position, &seat| {
        let number_of_occupied_adjacent_seats = grid::neighbors_including_diagonal(position)
            .into_iter()
            .filter(|&neighbor| seat_layout.get(neighbor).is_some_and(|&seat| seat == '#'))
            .count();
        if seat == 'L' && number_of_occupied_adjacent_seats == 0 {
            '#'
        } else if seat == '#' && number_of_occupied_adjacent_seats >= 4 {
            'L'
        } else {
            seat
        }
    })
}

fn visible_seats_from_all(seat_layout: &SeatLayout) -> Grid<Vec<Position>> {
    seat_layout.map(|position, _| visible_seats_from_one(seat_layout, position))
}

fn visible_seats_from_one(seat_layout: &SeatLayout, position: Position) -> Vec<Position> {
    grid::DIRECTIONS_INCLUDING_DIAGONAL
        .into_iter()
        .filter_map(|direction| visible_seat_in_direction(seat_layout, position, direction))
        .collect_vec()
}

fn visible_seat_in_direction(
    seat_layout: &SeatLayout,
    mut position: Position,
    direction: Direction,
) -> Option<Position> {
    position = position.add(direction);
    while let Some(seat) = seat_layout.get(position) {
        if ['L', '#'].contains(seat) {
            return Some(position);
        }
        position = position.add(direction);
    }
    None
}

fn next_considering_visible_seats(
    seat_layout: &SeatLayout,
    visible_seats: &Grid<Vec<Position>>,
) -> SeatLayout {
    seat_layout.map(|position, &seat| {
        let number_of_occupied_visible_seats = visible_seats[position]
            .iter()
            .filter(|&&visible| seat_layout[visible] == '#')
            .count();
        if seat == 'L' && number_of_occupied_visible_seats == 0 {
            '#'
        } else if seat == '#' && number_of_occupied_visible_seats >= 5 {
            'L'
        } else {
            seat
        }
    })
}

fn number_of_occupied_seats(seat_layout: &SeatLayout) -> usize {
    seat_layout
        .iter_row_major()
        .filter(|(_, &seat)| seat == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 11;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 37);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2386);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 26);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2091);
    }
}
