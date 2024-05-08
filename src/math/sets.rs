use std::hash::Hash;

use crate::HashSet;

pub fn intersection<T: Eq + Hash>(sets: impl IntoIterator<Item = HashSet<T>>) -> HashSet<T> {
    let mut sets = sets.into_iter();
    let mut intersection = sets
        .next()
        .expect("intersection should be computed from non-empty collections of sets");
    for set in sets {
        intersection.retain(|element| set.contains(element));
    }
    intersection
}
