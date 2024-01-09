use std::{fmt::Debug, str::FromStr};

pub fn number<N: FromStr>(number: &str) -> N
where
    <N as FromStr>::Err: Debug,
{
    number.parse().expect("string should be numeric")
}
