trait Submarine {
    fn forward(&mut self, value: Value);
    fn down(&mut self, value: Value);
    fn up(&mut self, value: Value);
    fn horizontal_position(&self) -> Value;
    fn depth(&self) -> Value;

    fn mov(&mut self, command: Command) {
        match command {
            Command::Forward(value) => self.forward(value),
            Command::Down(value) => self.down(value),
            Command::Up(value) => self.up(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Command {
    Forward(Value),
    Down(Value),
    Up(Value),
}

type Value = isize;

pub fn first(input: &str) -> String {
    let mut submarine = AimlessSubmarine::default();
    follow_commands(input, &mut submarine);
    (submarine.horizontal_position() * submarine.depth()).to_string()
}

#[derive(Debug, Clone, Copy, Default)]
struct AimlessSubmarine {
    horizontal_position: Value,
    depth: Value,
}

impl Submarine for AimlessSubmarine {
    fn forward(&mut self, distance: Value) {
        self.horizontal_position += distance;
    }

    fn down(&mut self, distance: Value) {
        self.depth += distance;
    }

    fn up(&mut self, distance: Value) {
        self.depth -= distance;
    }

    fn horizontal_position(&self) -> Value {
        self.horizontal_position
    }

    fn depth(&self) -> Value {
        self.depth
    }
}

pub fn second(input: &str) -> String {
    let mut submarine = AimfullSubmarine::default();
    follow_commands(input, &mut submarine);
    (submarine.horizontal_position() * submarine.depth()).to_string()
}

#[derive(Debug, Clone, Copy, Default)]
struct AimfullSubmarine {
    horizontal_position: Value,
    depth: Value,
    aim: Value,
}

impl Submarine for AimfullSubmarine {
    fn forward(&mut self, value: Value) {
        self.horizontal_position += value;
        self.depth += self.aim * value;
    }

    fn down(&mut self, value: Value) {
        self.aim += value;
    }

    fn up(&mut self, value: Value) {
        self.aim -= value;
    }

    fn horizontal_position(&self) -> Value {
        self.horizontal_position
    }

    fn depth(&self) -> Value {
        self.depth
    }
}

fn follow_commands(input: &str, submarine: &mut impl Submarine) {
    for command in commands(input) {
        submarine.mov(command);
    }
}

fn commands(input: &str) -> impl Iterator<Item = Command> + '_ {
    input.lines().map(command)
}

fn command(line: &str) -> Command {
    let (command, value) = line
        .split_once(' ')
        .expect("line should contain command and value separated by a space");
    let command = match command {
        "forward" => Command::Forward,
        "down" => Command::Down,
        "up" => Command::Up,
        _ => panic!("command should be 'forward', 'down', or 'up'"),
    };
    let value = value.parse().expect("value should be an integer");
    command(value)
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
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_694_130);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 900);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1_698_850_445);
    }
}
