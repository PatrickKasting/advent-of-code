use std::{fmt::Debug, str::FromStr, usize};

pub fn number<S: AsRef<str>, N: FromStr>(str: S) -> N
where
    <N as FromStr>::Err: Debug,
{
    str.as_ref().parse().expect("string should be numeric")
}

pub fn char_at<S: AsRef<str>>(str: S, index: usize) -> char {
    str.as_ref()
        .chars()
        .nth(index)
        .expect("char at given index should exist")
}

pub fn as_isize(value: usize) -> isize {
    value
        .try_into()
        .expect("value should be less than 'isize::MAX'")
}
