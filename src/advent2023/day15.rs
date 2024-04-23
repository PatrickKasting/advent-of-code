use easy_cast::Cast;

type HashMap<'label> = Vec<Bucket<'label>>;
type Bucket<'label> = Vec<(&'label str, FocalLength)>;
type FocalLength = usize;
type Hash = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
    Insertion(FocalLength),
    Removal,
}

pub fn first(input: &str) -> String {
    steps(input).map(hash).sum::<usize>().to_string()
}

pub fn second(input: &str) -> String {
    sum_of_focusing_powers(&hash_map(operations(input))).to_string()
}

fn sum_of_focusing_powers(hash_map: &HashMap) -> usize {
    let mut sum = 0;
    for (bucket_number, bucket) in (1..).zip(hash_map) {
        for (lens_number, (_, focal_length)) in (1..).zip(bucket) {
            sum += bucket_number * lens_number * *focal_length;
        }
    }
    sum
}

fn hash_map<'label>(steps: impl IntoIterator<Item = (&'label str, Operation)>) -> HashMap<'label> {
    let mut hash_map: HashMap = vec![Bucket::new(); 256];
    for (label, operation) in steps {
        let bucket = &mut hash_map[hash(label)];
        match operation {
            Operation::Insertion(focal_length) => {
                if let Some(position) = position(bucket, label) {
                    bucket[position].1 = focal_length;
                } else {
                    bucket.push((label, focal_length));
                }
            }
            Operation::Removal => {
                if let Some(position) = position(bucket, label) {
                    bucket.remove(position);
                }
            }
        }
    }
    hash_map
}

fn hash(str: &str) -> Hash {
    str.as_bytes()
        .iter()
        .fold(0_u8, |hash, &char| hash.wrapping_add(char).wrapping_mul(17))
        .cast()
}

fn position(bucket: &Bucket, label: &str) -> Option<usize> {
    bucket
        .iter()
        .position(|(label_in_map, _)| *label_in_map == label)
}

fn operations(sequence: &str) -> impl Iterator<Item = (&str, Operation)> {
    steps(sequence).map(operation)
}

fn operation(step: &str) -> (&str, Operation) {
    let operation_index = step
        .find(['-', '='])
        .expect("operation should contain '-' or '='");
    let (label, operation) = (&step[0..operation_index], &step[operation_index..]);
    let operation = match &operation[..1] {
        "-" => Operation::Removal,
        "=" => {
            let focal_length = operation[1..]
                .parse()
                .expect("focal length should be numerical");
            Operation::Insertion(focal_length)
        }
        _ => panic!("operation should be '-' or '='"),
    };
    (label, operation)
}

fn steps(sequence: &str) -> impl Iterator<Item = &str> {
    sequence.trim().split(',')
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 15;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1320);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 513_158);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 145);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 200_277);
    }
}
