use ahash::AHashSet;
use itertools::Itertools;

type TransparentPaper = AHashSet<Dot>;
type Dot = [Coordinate; 2];
type FoldInstruction = (FoldDirection, Coordinate);
type FoldDirection = usize;
type Coordinate = usize;

pub fn first_answer(input: &str) -> String {
    let (paper, instructions) = transparent_paper_and_fold_instructions(input);
    fold(paper, instructions[0]).len().to_string()
}

pub fn second_answer(input: &str) -> String {
    let (mut paper, instructions) = transparent_paper_and_fold_instructions(input);
    for instruction in instructions {
        paper = fold(paper, instruction);
    }
    display(paper)
}

fn fold(
    paper: TransparentPaper,
    (direction, fold_coordinate): FoldInstruction,
) -> TransparentPaper {
    paper
        .into_iter()
        .map(|mut dot| {
            if dot[direction] > fold_coordinate {
                dot[direction] -= 2 * (dot[direction] - fold_coordinate);
            }
            dot
        })
        .collect()
}

fn display(paper: TransparentPaper) -> String {
    let dots_sorted_by_row = paper
        .into_iter()
        .sorted_by_key(|&[column, row]| [row, column])
        .collect_vec();

    let mut display = String::new();
    let [mut current_column, mut current_row] = [0, 0];
    for [dot_column, dot_row] in dots_sorted_by_row {
        while current_row < dot_row {
            display.push('\n');
            current_row += 1;
            current_column = 0;
        }
        while current_column < dot_column {
            display.push('.');
            current_column += 1;
        }
        display.push('#');
        current_column += 1;
    }
    display
}

fn transparent_paper_and_fold_instructions(
    input: &str,
) -> (TransparentPaper, Vec<FoldInstruction>) {
    let (paper, instructions) = input
        .split_once("\n\n")
        .expect("dots and fold instructions should be separated by en empty line");
    (transparent_paper(paper), fold_instructions(instructions))
}

fn transparent_paper(str: &str) -> TransparentPaper {
    str.lines().map(dot).collect()
}

fn dot(line: &str) -> Dot {
    line.split(',')
        .map(|coordinate| coordinate.parse().expect("coordinate should be numeric"))
        .collect_vec()
        .try_into()
        .expect("line should contain two coordinates")
}

fn fold_instructions(str: &str) -> Vec<FoldInstruction> {
    str.lines().map(fold_instruction).collect_vec()
}

fn fold_instruction(line: &str) -> FoldInstruction {
    let (direction, coordinate) = line
        .split_once('=')
        .expect("instruction should contain an equal sign");
    let direction = match direction.as_bytes()[11] {
        b'x' => 0,
        b'y' => 1,
        _ => panic!("direction should be 'x' or 'y'"),
    };
    let coordinate = coordinate.parse().expect("coordinate should be numerical");
    (direction, coordinate)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 13;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 17);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 755);
    }

    #[test]
    fn second_answer_input() {
        let expected = "\
            ###..#....#..#...##.###..###...##...##\n\
            #..#.#....#.#.....#.#..#.#..#.#..#.#..#\n\
            ###..#....##......#.#..#.###..#..#.#\n\
            #..#.#....#.#.....#.###..#..#.####.#.##\n\
            #..#.#....#.#..#..#.#.#..#..#.#..#.#..#\n\
            ###..####.#..#..##..#..#.###..#..#..###\
        ";
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, expected);
    }
}
