use std::borrow::Borrow;

mod bst;

pub trait Tree<K: Ord, V>: IntoIterator<Item = (K, V)> {
    fn search<Q: Ord>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>;
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn remove<Q: Ord>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>;
}
