pub fn least_common_multiple(left: usize, right: usize) -> usize {
    left * (right / greatest_common_divisor(left, right))
}

pub fn greatest_common_divisor(left: usize, right: usize) -> usize {
    if right == 0 {
        left
    } else {
        greatest_common_divisor(right, left % right)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_cases;

    #[test]
    fn least_common_multiple() {
        let cases = [
            ((0, 7), 0),
            ((11, 0), 0),
            ((79, 79), 79),
            ((48, 18), 144),
            ((128, 96), 384),
            ((54, 24), 216),
            ((360, 210), 2520),
            ((108, 144), 432),
        ];
        test_cases(
            |(left, right)| super::least_common_multiple(left, right),
            cases,
        )
    }

    #[test]
    fn greatest_common_divisor() {
        let cases = [
            ((0, 7), 7),
            ((11, 0), 11),
            ((79, 79), 79),
            ((48, 18), 6),
            ((128, 96), 32),
            ((54, 24), 6),
            ((360, 210), 30),
            ((108, 144), 36),
        ];
        test_cases(
            |(left, right)| super::greatest_common_divisor(left, right),
            cases,
        )
    }
}
