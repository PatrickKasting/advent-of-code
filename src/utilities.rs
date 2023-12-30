use std::{fmt::Debug, str::FromStr};

pub fn number<Number: FromStr>(number: &str) -> Number
where
    <Number as FromStr>::Err: Debug,
{
    number.parse().expect("string should be numeric")
}
