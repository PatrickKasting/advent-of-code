use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

use easy_cast::{Cast, Conv};

use crate::number_theory::greatest_common_divisor;

type Numerator = isize;
type Denominator = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rational {
    numerator: Numerator,
    denominator: Denominator,
}

impl Rational {
    #[must_use]
    pub fn new(numerator: Numerator, denominator: Numerator) -> Option<Self> {
        (denominator != 0).then(|| canonical_form_negative_denominator(numerator, denominator))
    }

    #[must_use]
    pub fn numerator(self) -> Numerator {
        self.numerator
    }

    #[must_use]
    pub fn denominator(self) -> Denominator {
        self.denominator
    }

    #[must_use]
    pub fn is_integer(self) -> bool {
        self.denominator == 1
    }
}

impl PartialEq<Numerator> for Rational {
    fn eq(&self, &other: &Numerator) -> bool {
        self.is_integer() && self.numerator == other
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        Rational {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * Numerator::conv(rhs.denominator)
            + rhs.numerator * Numerator::conv(self.denominator);
        let denominator = self.denominator * rhs.denominator;
        canonical_form_positive_denominator(numerator, denominator)
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        canonical_form_positive_denominator(numerator, denominator)
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * Numerator::conv(rhs.denominator);
        let denominator = Numerator::conv(self.denominator) * rhs.numerator;
        canonical_form_negative_denominator(numerator, denominator)
    }
}

fn canonical_form_negative_denominator(numerator: Numerator, denominator: Numerator) -> Rational {
    canonical_form_positive_denominator(numerator * denominator.signum(), denominator.abs().cast())
}

fn canonical_form_positive_denominator(numerator: Numerator, denominator: Denominator) -> Rational {
    let gcd = greatest_common_divisor(numerator.abs().cast(), denominator);
    Rational {
        numerator: numerator / Numerator::conv(gcd),
        denominator: denominator / gcd,
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = self.numerator * Numerator::conv(other.denominator);
        let rhs = Numerator::conv(self.denominator) * other.numerator;
        lhs.cmp(&rhs)
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/{:?}", self.numerator, self.denominator)
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use infrastructure::test;

    use super::*;

    #[test]
    fn new() {
        let function = |(numerator, denominator)| Rational::new(numerator, denominator);
        let cases = [
            (
                (4, 6),
                Some(Rational {
                    numerator: 2,
                    denominator: 3,
                }),
            ),
            (
                (5, -7),
                Some(Rational {
                    numerator: -5,
                    denominator: 7,
                }),
            ),
            ((2, 0), None),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn add() {
        let function = |(lhs, rhs)| lhs + rhs;
        let cases = [
            (
                (
                    Rational {
                        numerator: 1,
                        denominator: 3,
                    },
                    Rational {
                        numerator: 4,
                        denominator: 6,
                    },
                ),
                Rational {
                    numerator: 1,
                    denominator: 1,
                },
            ),
            (
                (
                    Rational {
                        numerator: 7,
                        denominator: 9,
                    },
                    Rational {
                        numerator: 11,
                        denominator: 3,
                    },
                ),
                Rational {
                    numerator: 4 * 9 + 4,
                    denominator: 9,
                },
            ),
            (
                (
                    Rational {
                        numerator: 2,
                        denominator: 7,
                    },
                    Rational {
                        numerator: 1,
                        denominator: 5,
                    },
                ),
                Rational {
                    numerator: 17,
                    denominator: 35,
                },
            ),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn sub() {
        let function = |(lhs, rhs)| lhs - rhs;
        let cases = [
            (
                (
                    Rational {
                        numerator: 1,
                        denominator: 3,
                    },
                    Rational {
                        numerator: 4,
                        denominator: 6,
                    },
                ),
                Rational {
                    numerator: -1,
                    denominator: 3,
                },
            ),
            (
                (
                    Rational {
                        numerator: 7,
                        denominator: 9,
                    },
                    Rational {
                        numerator: 11,
                        denominator: 3,
                    },
                ),
                Rational {
                    numerator: -26,
                    denominator: 9,
                },
            ),
            (
                (
                    Rational {
                        numerator: 2,
                        denominator: 7,
                    },
                    Rational {
                        numerator: 1,
                        denominator: 5,
                    },
                ),
                Rational {
                    numerator: 3,
                    denominator: 35,
                },
            ),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn mul() {
        let function = |(lhs, rhs)| lhs * rhs;
        let cases = [
            (
                (
                    Rational {
                        numerator: 1,
                        denominator: 3,
                    },
                    Rational {
                        numerator: 4,
                        denominator: 6,
                    },
                ),
                Rational {
                    numerator: 2,
                    denominator: 9,
                },
            ),
            (
                (
                    Rational {
                        numerator: -7,
                        denominator: 9,
                    },
                    Rational {
                        numerator: -11,
                        denominator: 3,
                    },
                ),
                Rational {
                    numerator: 77,
                    denominator: 27,
                },
            ),
            (
                (
                    Rational {
                        numerator: 2,
                        denominator: 7,
                    },
                    Rational {
                        numerator: 1,
                        denominator: 5,
                    },
                ),
                Rational {
                    numerator: 2,
                    denominator: 35,
                },
            ),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn div() {
        let function = |(lhs, rhs)| lhs / rhs;
        let cases = [
            (
                (
                    Rational {
                        numerator: -1,
                        denominator: 3,
                    },
                    Rational {
                        numerator: 4,
                        denominator: 6,
                    },
                ),
                Rational {
                    numerator: -1,
                    denominator: 2,
                },
            ),
            (
                (
                    Rational {
                        numerator: 7,
                        denominator: 9,
                    },
                    Rational {
                        numerator: -11,
                        denominator: 3,
                    },
                ),
                Rational {
                    numerator: -7,
                    denominator: 33,
                },
            ),
            (
                (
                    Rational {
                        numerator: -2,
                        denominator: 7,
                    },
                    Rational {
                        numerator: -1,
                        denominator: 5,
                    },
                ),
                Rational {
                    numerator: 10,
                    denominator: 7,
                },
            ),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn ord() {
        let function = |(lhs, rhs): (Rational, Rational)| lhs.cmp(&rhs);
        let cases = [
            (
                (
                    Rational {
                        numerator: -1,
                        denominator: 3,
                    },
                    Rational {
                        numerator: 4,
                        denominator: 6,
                    },
                ),
                Ordering::Less,
            ),
            (
                (
                    Rational {
                        numerator: 5,
                        denominator: 8,
                    },
                    Rational {
                        numerator: 6,
                        denominator: 7,
                    },
                ),
                Ordering::Less,
            ),
            (
                (
                    Rational {
                        numerator: -2,
                        denominator: 7,
                    },
                    Rational {
                        numerator: -2,
                        denominator: 7,
                    },
                ),
                Ordering::Equal,
            ),
            (
                (
                    Rational {
                        numerator: 7,
                        denominator: 9,
                    },
                    Rational {
                        numerator: -11,
                        denominator: 3,
                    },
                ),
                Ordering::Greater,
            ),
            (
                (
                    Rational {
                        numerator: -7,
                        denominator: 11,
                    },
                    Rational {
                        numerator: -8,
                        denominator: 10,
                    },
                ),
                Ordering::Greater,
            ),
        ];
        test::cases(function, cases);
    }
}
