use std::{fmt::Debug, str::FromStr};

pub fn parse<S: AsRef<str>, N: FromStr>(str: S) -> N
where
    <N as FromStr>::Err: Debug,
{
    str.as_ref().parse().expect("string should be parsable")
}
