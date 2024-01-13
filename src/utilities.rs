use std::{fmt::Debug, str::FromStr};

pub fn number<S: AsRef<str>, N: FromStr>(number: S) -> N
where
    <N as FromStr>::Err: Debug,
{
    number.as_ref().parse().expect("string should be numeric")
}

pub fn char_at<S: AsRef<str>>(str: S, index: usize) -> char {
    str.as_ref()
        .chars()
        .nth(index)
        .expect("char at given index should exist")
}
