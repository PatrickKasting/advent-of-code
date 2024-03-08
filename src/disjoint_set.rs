use std::{collections::HashMap, hash::Hash};

pub struct DisjointSet<T: Copy + Eq + Hash>(HashMap<T, T>);

impl<T: Copy + Eq + Hash> DisjointSet<T> {
    pub fn new(elements: impl IntoIterator<Item = T>) -> Self {
        Self(HashMap::from_iter(
            elements.into_iter().map(|element| (element, element)),
        ))
    }

    pub fn union(&mut self, left: T, right: T) {
        let left_representative = self.find_set(left);
        let right_representative = self.find_set(right);
        for (_, representative) in self.0.iter_mut() {
            if *representative == left_representative {
                *representative = right_representative;
            }
        }
    }

    pub fn find_set(&self, element: T) -> T {
        self.0[&element]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disjoint_set() {
        let elements = 'a'..='g';
        let mut disjoint_set = DisjointSet::new(elements.clone());
        for element in elements {
            assert_eq!(disjoint_set.find_set(element), element);
        }
        disjoint_set.union('c', 'g');
        disjoint_set.union('c', 'c');
        assert_eq!(disjoint_set.find_set('c'), disjoint_set.find_set('g'));
        disjoint_set.union('e', 'f');
        disjoint_set.union('b', 'e');
        assert_eq!(disjoint_set.find_set('b'), disjoint_set.find_set('f'));
        disjoint_set.union('a', 'd');
        disjoint_set.union('e', 'a');
        assert_eq!(disjoint_set.find_set('d'), disjoint_set.find_set('f'));
        assert_ne!(disjoint_set.find_set('a'), disjoint_set.find_set('c'),)
    }
}
