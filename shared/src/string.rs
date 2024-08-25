use std::sync::OnceLock;

use itertools::Itertools;
use regex::Regex;

/// # Panics
///
/// Panics if the internal regex is incorrect.
pub fn usizes(haystack: &str) -> Vec<usize> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| Regex::new(r"\d+").expect("regex should be valid"));
    matches(regex, haystack)
        .map(|str| str.parse().expect("match should be parseable"))
        .collect_vec()
}

/// # Panics
///
/// Panics if the internal regex is incorrect.
pub fn isizes(haystack: &str) -> Vec<isize> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| Regex::new(r"-?\d+").expect("regex should be valid"));
    matches(regex, haystack)
        .map(|str| str.parse().expect("match should be parseable"))
        .collect_vec()
}

pub fn matches<'regex, 'haystack: 'regex>(
    regex: &'regex Regex,
    haystack: &'haystack str,
) -> impl Iterator<Item = &'haystack str> + 'regex {
    regex.find_iter(haystack).map(|mat| mat.as_str())
}
