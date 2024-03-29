use std::borrow::Borrow;

use node::Node;

use crate::trees::Tree;

mod node;

pub struct Bst<K: Ord, V> {
    root: Option<Node<K, V>>,
    length: usize,
}

impl<K: Ord, V> Bst<K, V> {
    pub const fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }
    pub const fn length(&self) -> usize {
        self.length
    }
}

impl<K: Ord, V> Tree<K, V> for Bst<K, V> {
    fn search<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.root.as_ref()?.search(k)
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        todo!()
    }

    fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        todo!()
    }
}

impl<K: Ord, V> IntoIterator for Bst<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct IntoIter<K: Ord, V>(Vec<Node<K, V>>);

impl<K: Ord, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
