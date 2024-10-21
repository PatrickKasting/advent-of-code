use std::mem;

#[must_use]
pub fn least_common_multiple(lhs: usize, rhs: usize) -> usize {
    lhs / greatest_common_divisor(lhs, rhs) * rhs
}

#[must_use]
pub fn greatest_common_divisor(mut lhs: usize, mut rhs: usize) -> usize {
    if rhs > lhs {
        mem::swap(&mut lhs, &mut rhs);
    }
    if rhs == 0 {
        lhs
    } else {
        greatest_common_divisor(rhs, lhs % rhs)
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::test;

    #[test]
    fn least_common_multiple() {
        let function = |(lhs, rhs)| super::least_common_multiple(lhs, rhs);
        let cases = [
            ((4, 6), 12),
            ((6, 4), 12),
            ((21, 6), 42),
            ((6, 21), 42),
            ((48, 180), 720),
            ((180, 48), 720),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn greatest_common_divisor() {
        let function = |(lhs, rhs)| super::greatest_common_divisor(lhs, rhs);
        let cases = [
            ((48, 18), 6),
            ((18, 48), 6),
            ((42, 56), 14),
            ((56, 42), 14),
            ((54, 24), 6),
            ((24, 54), 6),
        ];
        test::cases(function, cases);
    }
}
