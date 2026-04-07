use std::{
    fmt::{Debug, DebugSet},
    hash::Hash,
};

use ahash::AHashMap;

pub struct Trie<T> {
    root: Node<T>,
}

struct Node<T> {
    is_end: bool,
    children: AHashMap<T, Node<T>>,
}

impl<T> Trie<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node {
                is_end: false,
                children: AHashMap::new(),
            },
        }
    }
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Eq + Hash> Trie<T> {
    pub fn insert(&mut self, sequence: impl IntoIterator<Item = T>) -> bool {
        let mut node = &mut self.root;
        for element in sequence {
            node = node.children.entry(element).or_insert_with(|| Node {
                is_end: false,
                children: AHashMap::new(),
            });
        }
        let newly_inserted = !node.is_end;
        node.is_end = true;
        newly_inserted
    }

    pub fn contains(&self, sequence: impl IntoIterator<Item = T>) -> bool {
        let mut node = &self.root;
        for element in sequence {
            match node.children.get(&element) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }

    pub fn prefix_lengths<S: IntoIterator<Item = T>>(
        &self,
        sequence: S,
    ) -> PrefixLengths<'_, T, S::IntoIter> {
        PrefixLengths {
            sequence: sequence.into_iter(),
            prefix: vec![],
            node: Some(&self.root),
        }
    }
}

pub struct PrefixLengths<'trie, T, S> {
    sequence: S,
    prefix: Vec<T>,
    node: Option<&'trie Node<T>>,
}

impl<T: Copy + Eq + Hash, S: Iterator<Item = T>> Iterator for PrefixLengths<'_, T, S> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut prefix_length = None;
        while prefix_length.is_none() {
            prefix_length = self.node?.is_end.then_some(self.prefix.len());
            self.node = if let Some(node) = self.node
                && let Some(element) = self.sequence.next()
            {
                self.prefix.push(element);
                node.children.get(&element)
            } else {
                None
            };
        }
        prefix_length
    }
}

impl<
    T: Copy + Eq + Hash,
    Sequence: IntoIterator<Item = T>,
    Sequences: IntoIterator<Item = Sequence>,
> From<Sequences> for Trie<T>
{
    fn from(sequences: Sequences) -> Self {
        let mut trie = Self::new();
        for sequence in sequences {
            trie.insert(sequence);
        }
        trie
    }
}

impl<T: Debug + Copy + Eq + Hash> Debug for Trie<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_node<T: Debug + Copy + Eq + Hash>(
            debug_set: &mut DebugSet,
            sequence: &mut Vec<T>,
            node: &Node<T>,
        ) {
            if node.is_end {
                debug_set.entry(&sequence);
            }
            for (&element, child) in &node.children {
                sequence.push(element);
                fmt_node(debug_set, sequence, child);
                sequence.pop();
            }
        }

        let mut debug_set = f.debug_set();
        fmt_node(&mut debug_set, &mut vec![], &self.root);
        debug_set.finish()
    }
}

#[cfg(test)]
mod tests {
    use ahash::AHashSet;
    use rand::{RngExt, distr::Alphabetic};

    use super::*;

    #[test]
    fn behaves_as_set() {
        let mut expected: AHashSet<String> = AHashSet::new();
        let mut actual: Trie<char> = Trie::new();
        for _ in 0..1_000_000 {
            let string = random_string(4);
            let insert = rand::rng().random_bool(0.9);
            if insert {
                assert_eq!(
                    expected.insert(string.clone()),
                    actual.insert(string.chars()),
                    "expected and actual should agree on insertion of '{string}'"
                );
            } else {
                assert_eq!(
                    expected.contains(&string),
                    actual.contains(string.chars()),
                    "expected and actual should agree on membership of '{string}'"
                );
            }
        }
    }

    fn random_string(max_len: usize) -> String {
        let len = rand::rng().random_range(..=max_len);
        rand::rng()
            .sample_iter(&Alphabetic)
            .take(len)
            .map(char::from)
            .collect()
    }

    #[test]
    fn prefix_lengths_with_short_elements() {
        let elements = ["", "ab", "bc", "abc", "abcde", "bcd", "c"];
        let trie = Trie::from(elements.map(str::chars));
        let actual = trie.prefix_lengths("abcdef".chars());
        let expected = [0, 2, 3, 5];
        assert!(itertools::equal(actual, expected));
    }

    #[test]
    fn prefix_lengths_with_short_query() {
        let elements = ["", "ab", "bc", "abc", "abcde", "bcd", "c", "abcdefg"];
        let trie = Trie::from(elements.map(str::chars));
        let actual = trie.prefix_lengths("abcdef".chars());
        let expected = [0, 2, 3, 5];
        assert!(itertools::equal(actual, expected));
    }
}
