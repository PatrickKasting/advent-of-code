/// Returns the answer to the first puzzle of day 24.
///
/// # Correctness
///
/// For each digit in the given model number, the MONAD performs the same routine with different
/// parameters. This routine uses the register `z` as a persistent accumulating variable and the
/// registers `w`, `x`, and `y` as local variables. The routine is as follows, where `divisor`,
/// `shift`, and `addition` are parameters that differ from iteration to iteration:
///
/// ```
/// fn process_digit(
///     z: &mut isize,
///     digit: isize,
///     divisor: isize,
///     shift: isize,
///     addition: isize
/// ) {
///     assert!([1, 26].contains(&divisor));
///     let top_shifted = *z % 26 + shift;
///     *z /= divisor;
///     if top_shifted != digit {
///         *z = *z * 26 + digit + addition
///     }
/// }
/// ```
///
/// Notice that `z` is always multiplied or divided by `26` (or `1`) before any additions occur.
/// This is because `z` is actually a stack of integers in the range `0..26`: If `divisor == 26`,
/// an integer is popped from the stack and if `top_shifted != digit`, the integer
/// `digit + addition` is pushed onto the stack.
///
/// A model number is valid if `z == 0` after the MONAD has finished. That is, the stack must be
/// empty or all integers on the stack must be `0`.
///
/// Below, the values of the parameters are listed. Notice that `divisor` is `26` seven times, which
/// results in seven pops from the stack. During the other seven iterations, `shift >= 10`, meaning
/// `top_shifted != digit` because `digit < 10`. Thus, we push `digit + addition` onto the stack
/// during these iterations. Because it's always the case that `addition > 0`, we never push zeros
/// onto the stack and thus, the stack must end up empty in order to accept the model number.
///
/// | iteration  |  0 |  1 |  2 |  3 |  4 |   5 |  6 |  7 |   8 |  9 |  10 | 11 |  12 | 13 |
/// |:-----------|---:|---:|---:|---:|---:|----:|---:|---:|----:|---:|----:|---:|----:|---:|
/// | `divisor`  |  1 |  1 |  1 |  1 |  1 |  26 | 26 |  1 |  26 |  1 |  26 | 26 |  26 | 26 |
/// | `shift`    | 10 | 10 | 14 | 11 | 14 | -14 |  0 | 10 | -10 | 13 | -12 | -3 | -11 | -2 |
/// | `addition` |  2 |  4 |  8 |  7 | 12 |   7 | 10 | 14 |   2 |  6 |   8 | 11 |   5 | 11 |
///
/// The seven unavoidable pushes and the seven unavoidable pops yield an empty stack, if no other
/// pushes occur. That is, we must have `top_shifted == digit` during all iterations where
/// `divisor == 26`.
///
/// Let's consider iteration 13, which should handle the number `digit[0] + additon[0]` pushed
/// during iteration 0: We have `top_shifted[13] == digit[0] + additon[0] + shift[13]`, which
/// reduces to `top_shifted[13] == digit[0]` because `additon[0] == -shift[13]`. In order to avoid a
/// stack push, we must have `top_shifted[13] == digit[13]`, which is equivalent to
/// `digit[0] == digit[13]`. Thus, as long as the first digit and the last digit of the model number
/// are identical, we avoid a push during iteration 13. Since we are interested in the largest
/// accepted model number, we choose `digit[0] == digit[13] == 9`.
///
/// Now, let's consider iteration 12, which should handle the number `digit[1] + addition[1]` pushed
/// during iteration 1: We have `top_shifted[12] == digit[1] + addition[1] + shift[12]`, which
/// implies `digit[12] == digit[1] - 7`. Since we are looking for the largest accepted model number,
/// we choose `digit[1] == 9` and `digit[12] == 2`.
///
/// If we perform this analysis for every iteration, we find that the largest accepted model number
/// is `99_429_795_993_929`.
pub fn first_answer(_: &str) -> String {
    99_429_795_993_929_isize.to_string()
}

/// Returns the answer to the first puzzle of day 24.
///
/// # Correctness
///
/// An analysis similar to that of [`first`] yields that the smallest accepted model number is
/// `18_113_181_571_611`.
pub fn second_answer(_: &str) -> String {
    18_113_181_571_611_isize.to_string()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 24;

    #[test]
    fn first_answer_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            99_429_795_993_929_isize,
        );
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            18_113_181_571_611_isize,
        );
    }
}
