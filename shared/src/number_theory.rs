use easy_cast::Cast;
use itertools::Itertools;

#[must_use]
pub fn chinese_remainder_theorem(
    congruences: impl Iterator<Item = (isize, isize)> + Clone,
) -> isize {
    debug_assert_chinese_remainder_theorem_properties(congruences.clone());

    let n: isize = congruences.clone().map(|(_, modulo)| modulo).product();
    let ms = congruences.clone().map(|(_, modulo)| n / modulo);
    congruences
        .zip_eq(ms)
        .map(|((remainder, modulo), m)| {
            let [_, v] = bezouts_identity(modulo, m);
            remainder * v * m
        })
        .sum::<isize>()
        .rem_euclid(n)
}

fn debug_assert_chinese_remainder_theorem_properties(
    congruences: impl Iterator<Item = (isize, isize)> + Clone,
) {
    let modulos = congruences.map(|(_, modulo)| modulo);

    let are_modulos_greater_than_one = modulos.clone().all(|modulo| modulo > 1);
    debug_assert!(
        are_modulos_greater_than_one,
        "modulos must be greater than one"
    );

    let are_modulos_pairwise_coprime = modulos
        .combinations(2)
        .all(|pair| greatest_common_divisor(pair[0].cast(), pair[1].cast()) == 1);
    debug_assert!(
        are_modulos_pairwise_coprime,
        "modulos must be pairwise coprime"
    );
}

fn bezouts_identity(mut a: isize, mut b: isize) -> [isize; 2] {
    let mut swapped = false;
    if a < b {
        [a, b] = [b, a];
        swapped = true;
    }

    let mut quotients = vec![];
    while b != 0 {
        quotients.push(a / b);
        [a, b] = [b, a % b];
    }

    let mut factors = quotients
        .into_iter()
        .rev()
        .fold([1, 0], |[smaller, greater], quotient| {
            [greater, smaller + greater * -quotient]
        });

    if swapped {
        factors.reverse();
    }
    factors
}

#[must_use]
pub fn least_common_multiple(lhs: usize, rhs: usize) -> usize {
    lhs / greatest_common_divisor(lhs, rhs) * rhs
}

#[must_use]
pub fn greatest_common_divisor(mut lhs: usize, mut rhs: usize) -> usize {
    if rhs > lhs {
        [lhs, rhs] = [rhs, lhs];
    }
    if rhs == 0 {
        lhs
    } else {
        greatest_common_divisor(rhs, lhs % rhs)
    }
}

#[must_use]
pub fn number_of_decimal_digits(mut number: isize) -> u32 {
    let mut number_of_digits = 0;
    while number != 0 {
        number /= 10;
        number_of_digits += 1;
    }
    number_of_digits
}

#[cfg(test)]
mod tests {
    use easy_cast::Cast;
    use infrastructure::test;

    #[test]
    fn chinese_remainder_theorem() {
        let function =
            |congruences: &[_]| super::chinese_remainder_theorem(congruences.iter().copied());
        let cases = [
            (&[(8, 15)][..], 8),
            (&[(8 + 15, 15)][..], 8),
            (&[(1, 3), (2, 4)][..], 10),
            (&[(1, 3), (2 - 4, 4)][..], 10),
            (&[(4, 17), (1, 15), (14, 28)][..], 4186),
            (&[(4 + 17, 17), (1 - 15, 15), (14 - 2 * 28, 28)][..], 4186),
            (&[(201, 289), (54, 143), (9, 18), (7, 53)][..], 15_566_319),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn bezouts_identity() {
        let cases = [
            [9, 0],
            [0, 9],
            [9, 9],
            [9, 3],
            [3, 9],
            [20, 3],
            [3, 20],
            [15, 4],
            [12, 5],
            [1785, 546],
            [546, 1785],
        ];
        for case @ [a, b] in cases {
            let [k, l] = super::bezouts_identity(a, b);
            assert_eq!(
                a * k + b * l,
                super::greatest_common_divisor(a.cast(), b.cast()).cast(),
                "answer to case '{case:?}' should result in the greatest common divisor"
            );
        }
    }

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

    #[test]
    fn number_of_decimal_digits() {
        let cases = [
            (1, 1),
            (9, 1),
            (10, 2),
            (9_827_007, 7),
            (1_000_000_000, 10),
            (-1, 1),
            (-9, 1),
            (-10, 2),
            (-9_827_007, 7),
            (-1_000_000_000, 10),
        ];
        test::cases(super::number_of_decimal_digits, cases);
    }
}
