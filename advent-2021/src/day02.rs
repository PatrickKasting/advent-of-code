trait Submarine {
    fn forward(&mut self, distance: usize);
    fn up(&mut self, distance: usize);
    fn down(&mut self, distance: usize);
}

struct FirstSubmarine {
    horizontal_position: usize,
    depth: usize,
}

impl Submarine for FirstSubmarine {
    fn forward(&mut self, distance: usize) {
        self.horizontal_position += distance;
    }

    fn up(&mut self, distance: usize) {
        self.depth -= distance;
    }

    fn down(&mut self, distance: usize) {
        self.depth += distance;
    }
}

struct SecondSubmarine {
    horizontal_position: usize,
    depth: usize,
    aim: usize,
}

impl Submarine for SecondSubmarine {
    fn forward(&mut self, distance: usize) {
        self.horizontal_position += distance;
        self.depth += self.aim * distance;
    }

    fn up(&mut self, distance: usize) {
        self.aim -= distance;
    }

    fn down(&mut self, distance: usize) {
        self.aim += distance;
    }
}

pub fn first(input: &str) -> String {
    let mut submarine = FirstSubmarine {
        horizontal_position: 0,
        depth: 0,
    };
    commands(input, &mut submarine);
    (submarine.horizontal_position * submarine.depth).to_string()
}

pub fn second(input: &str) -> String {
    let mut submarine = SecondSubmarine {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };
    commands(input, &mut submarine);
    (submarine.horizontal_position * submarine.depth).to_string()
}

fn commands(commands: &str, submarine: &mut impl Submarine) {
    for command in commands.lines() {
        let (direction, distance) = command
            .split_once(' ')
            .expect("command should contain a space");
        let distance: usize = distance
            .parse()
            .expect("last part of command should be a positive integer");
        match direction {
            "forward" => submarine.forward(distance),
            "up" => submarine.up(distance),
            "down" => submarine.down(distance),
            _ => panic!("command should be 'forward', 'up', or 'down'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 2;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 150);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1694130);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 900);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1698850445);
    }
}
