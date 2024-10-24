use ahash::AHashMap;
use shared::{string::isizes, vector::Vector};

type Diagram = AHashMap<Point, usize>;
type Line = [Point; 2];
type Point = [Coordinate; 2];
type Coordinate = isize;

pub fn first_answer(input: &str) -> String {
    let lines = lines(input);
    let lines = lines.filter(|&line| !is_diagonal(line));
    number_of_overlap_points(lines).to_string()
}

pub fn second_answer(input: &str) -> String {
    let lines = lines(input);
    number_of_overlap_points(lines).to_string()
}

fn is_diagonal([from, to]: Line) -> bool {
    from[0] != to[0] && from[1] != to[1]
}

fn number_of_overlap_points(lines: impl Iterator<Item = Line>) -> usize {
    let mut diagram = AHashMap::new();
    for line in lines {
        add(line, &mut diagram);
    }
    diagram
        .into_iter()
        .filter(|&(_, number_of_overlaps)| number_of_overlaps >= 2)
        .count()
}

fn add([from, to]: Line, diagram: &mut Diagram) {
    *diagram.entry(from).or_default() += 1;

    let direction = to.sub(from).map(Coordinate::signum);
    let mut current = from;
    while current != to {
        current = current.add(direction);
        *diagram.entry(current).or_default() += 1;
    }
}

fn lines(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().map(line)
}

fn line(line: &str) -> Line {
    let (left, right) = line
        .split_once(" -> ")
        .expect("line should contain an arrow");
    [point(left), point(right)]
}

fn point(str: &str) -> Point {
    isizes(str)
        .try_into()
        .expect("point should consist of two coordinates")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 5;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 8622);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 12);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 22037);
    }
}
