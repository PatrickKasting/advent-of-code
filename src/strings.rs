use std::{fmt::Debug, str::FromStr, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;

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

pub fn matches<'regex, 'haystack: 'regex>(
    regex: &'regex Regex,
    haystack: &'haystack str,
) -> impl Iterator<Item = &'haystack str> + 'regex {
    regex.find_iter(haystack).map(|mat| mat.as_str())
}

pub fn usizes(haystack: &str) -> Vec<usize> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| Regex::new(r"\d+").expect("regex should be valid"));
    matches(regex, haystack).map(parse).collect_vec()
}

pub fn isizes(haystack: &str) -> Vec<isize> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| Regex::new(r"-?\d+").expect("regex should be valid"));
    matches(regex, haystack).map(parse).collect_vec()
}
