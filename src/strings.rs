use std::{fmt::Debug, str::FromStr};

pub fn parse<S: AsRef<str>, N: FromStr>(str: S) -> N
where
    <N as FromStr>::Err: Debug,
{
    str.as_ref().parse().expect("string should be parsable")
}

pub fn char_at<S: AsRef<str>>(str: S, index: usize) -> char {
    str.as_ref()
        .chars()
        .nth(index)
        .unwrap_or_else(|| panic!("string should be at least {index} characters long"))
}
