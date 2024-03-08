use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisjointSet<T: Copy + Eq + Hash> {
    parents: HashMap<T, T>,
    tree_sizes: HashMap<T, usize>,
}

impl<T: Copy + Eq + Hash> DisjointSet<T> {
    pub fn new(elements: impl IntoIterator<Item = T>) -> Self {
        let parents: HashMap<T, T> = elements
            .into_iter()
            .map(|element| (element, element))
            .collect();
        let tree_sizes = parents.keys().map(|&element| (element, 1)).collect();
        Self {
            parents,
            tree_sizes,
        }
    }

    fn link(&mut self, left_tree_root: T, right_tree_root: T) {
        let left_tree_size = self.tree_sizes[&left_tree_root];
        let right_tree_size = self.tree_sizes[&right_tree_root];
        let (smallest_tree_root, biggest_tree_root) = if left_tree_size < right_tree_size {
            (left_tree_root, right_tree_root)
        } else {
            (right_tree_root, left_tree_root)
        };
        self.parents.insert(smallest_tree_root, biggest_tree_root);
        self.tree_sizes
            .insert(biggest_tree_root, left_tree_size + right_tree_size);
    }

    pub fn union(&mut self, left: T, right: T) {
        let left_tree_root = self.find_set(left);
        let right_tree_root = self.find_set(right);
        if left_tree_root != right_tree_root {
            self.link(left_tree_root, right_tree_root)
        }
    }

    pub fn find_set(&mut self, element: T) -> T {
        let mut root = element;
        let parent = self.parents[&element];
        if parent != element {
            root = self.find_set(parent);
            self.parents.insert(element, root);
        }
        root
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
        assert_ne!(disjoint_set.find_set('a'), disjoint_set.find_set('c'));
    }
}
