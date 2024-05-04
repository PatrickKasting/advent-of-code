use std::{
    f64,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_traits::NumCast;

pub trait Addition {
    fn add(self, rhs: Self) -> Self;
}

impl<T, const N: usize> Addition for [T; N]
where
    T: Copy + Add<T, Output = T>,
{
    fn add(mut self, rhs: Self) -> Self {
        for (index, element) in self.iter_mut().enumerate() {
            *element = *element + rhs[index];
        }
        self
    }
}

pub trait Subtraction {
    fn sub(self, rhs: Self) -> Self;
}

impl<T, const N: usize> Subtraction for [T; N]
where
    T: Copy + Sub<T, Output = T>,
{
    fn sub(mut self, rhs: Self) -> Self {
        for (index, element) in self.iter_mut().enumerate() {
            *element = *element - rhs[index];
        }
        self
    }
}

pub trait ScalarMultiplication {
    type Scalar;
    fn mul(self, scalar: Self::Scalar) -> Self;
}

impl<T, const N: usize> ScalarMultiplication for [T; N]
where
    T: Copy + Mul<T, Output = T>,
{
    type Scalar = T;

    fn mul(mut self, scalar: Self::Scalar) -> Self {
        for element in &mut self {
            *element = *element * scalar;
        }
        self
    }
}

pub trait ScalarDivision {
    type Scalar;
    #[allow(dead_code)]
    fn div(self, scalar: Self::Scalar) -> Self;
}

impl<T, const N: usize> ScalarDivision for [T; N]
where
    T: Copy + Div<T, Output = T>,
{
    type Scalar = T;

    fn div(mut self, scalar: Self::Scalar) -> Self {
        for element in &mut self {
            *element = *element / scalar;
        }
        self
    }
}

pub trait Negation {
    fn neg(self) -> Self;
}

impl<T, const N: usize> Negation for [T; N]
where
    T: Neg<Output = T>,
{
    fn neg(self) -> Self {
        self.map(|element| -element)
    }
}

pub trait DotProduct {
    type Scalar;
    fn dot(self, rhs: Self) -> Self::Scalar;
}

impl<T, const N: usize> DotProduct for [T; N]
where
    T: Default + Add<T, Output = T> + Mul<T, Output = T>,
{
    type Scalar = T;

    fn dot(self, rhs: Self) -> Self::Scalar {
        self.into_iter()
            .zip(rhs)
            .fold(T::default(), |sum, (left, right)| sum + left * right)
    }
}

pub trait Magnitude {
    fn magnitude(self) -> f64;
}

impl<T, const N: usize> Magnitude for [T; N]
where
    T: Copy,
    [T; N]: DotProduct,
    <[T; N] as DotProduct>::Scalar: NumCast,
{
    fn magnitude(self) -> f64 {
        let dot = <f64 as NumCast>::from(self.dot(self)).expect("number should convert to 'f64'");
        dot.sqrt()
    }
}

pub trait Unit {
    type Unit;
    fn unit(self) -> Self::Unit;
}

impl<T, const N: usize> Unit for [T; N]
where
    T: Copy + NumCast,
    [T; N]: Magnitude,
{
    type Unit = [f64; N];
    fn unit(self) -> Self::Unit {
        let magnitude = self.magnitude();
        self.map(|element| {
            let floating = <f64 as NumCast>::from(element).expect("number should convert to 'f64'");
            floating / magnitude
        })
    }
}

pub trait CrossProduct {
    fn cross(self, rhs: Self) -> Self;
}

impl<T> CrossProduct for [T; 3]
where
    T: Copy + Sub<T, Output = T> + Mul<T, Output = T>,
{
    fn cross(self, rhs: Self) -> Self {
        [
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ]
    }
}

pub trait RotationInTwoDimensions {
    fn left(self) -> Self;
    fn right(self) -> Self;
}

impl<T> RotationInTwoDimensions for [T; 2]
where
    T: Neg<Output = T>,
{
    fn left(self) -> Self {
        let [x, y] = self;
        [-y, x]
    }

    fn right(self) -> Self {
        let [x, y] = self;
        [y, -x]
    }
}

pub trait Angle {
    fn angle(self, to: Self) -> f64;
}

impl<T> Angle for [T; 2]
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + NumCast,
{
    fn angle(self, to: Self) -> f64 {
        let dot = <f64 as NumCast>::from(self[0] * to[0] + self[1] * to[1])
            .expect("number should convert to 'f64'");
        let determinant = <f64 as NumCast>::from(self[0] * to[1] - self[1] * to[0])
            .expect("number should convert to 'f64'");
        determinant.atan2(dot)
    }
}

pub fn round<T: NumCast, const N: usize>(vector: [f64; N]) -> [T; N] {
    vector.map(|element| {
        <T as NumCast>::from(element.round()).expect("number should convert from 'f64'")
    })
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;

    use crate::floating::ApproxEq;

    use super::*;

    #[test]
    fn addition() {
        let actual = [1, 2, 3].add([-6, -4, -2]);
        assert_eq!(actual, [-5, -2, 1]);
    }

    #[test]
    fn subtraction() {
        let actual = [1, 2, 3].sub([-6, -4, -2]);
        assert_eq!(actual, [7, 6, 5]);
    }

    #[test]
    fn scalar_multiplication() {
        let actual = [-3, 4, 0, -3, 2].mul(2);
        assert_eq!(actual, [-6, 8, 0, -6, 4]);
    }

    #[test]
    fn scalar_division() {
        let actual = [-4, 4, 0, -2, 8].div(2);
        assert_eq!(actual, [-2, 2, 0, -1, 4]);
    }

    #[test]
    fn negation() {
        let vector = [-6, -4, -2];
        assert_eq!(vector.neg(), [6, 4, 2]);
    }

    #[test]
    fn dot_product() {
        let actual = [1, 3, -5, 2].dot([4, -2, -1, 2]);
        assert_eq!(actual, 7);
    }

    #[test]
    fn magnitude() {
        let actual = [-3, 4].magnitude();
        assert!(actual.approx_eq(5.0));
    }

    #[test]
    fn unit() {
        let actual = [-3, 3].unit();
        let expected = [-FRAC_1_SQRT_2, FRAC_1_SQRT_2];
        for (coordinate, expected) in actual.into_iter().zip(expected) {
            assert!(coordinate.approx_eq(expected));
        }
    }

    #[test]
    fn cross() {
        let actual = [3, -3, 1].cross([4, 9, 2]);
        assert_eq!(actual, [-15, -2, 39]);
    }

    #[test]
    fn left() {
        let actual = [2, 1].left();
        assert_eq!(actual, [-1, 2]);
    }

    #[test]
    fn right() {
        let actual = [-1, 2].right();
        assert_eq!(actual, [2, 1]);
    }

    #[test]
    fn angle() {
        let cases = [
            ([[1, 0], [3, 0]], 0.0),
            ([[2, 0], [4, 4]], FRAC_PI_4),
            ([[3, 0], [0, 5]], FRAC_PI_2),
            ([[4, 0], [-1, 1]], 3.0 * FRAC_PI_4),
            ([[5, 0], [-2, 0]], PI),
            ([[6, 0], [-3, -3]], -3.0 * FRAC_PI_4),
            ([[7, 0], [0, -4]], -FRAC_PI_2),
            ([[8, 0], [5, -5]], -FRAC_PI_4),
        ];
        for ([from, to], expected) in cases {
            let actual = from.angle(to) / PI;
            let expected = expected / PI;
            assert!(
                actual.approx_eq(expected),
                "actual angle {actual:.2}π from {from:?} to {to:?} should equal {expected:.2}π"
            );
        }
    }
}
