use std::borrow::Borrow;
use std::mem;

#[derive(Debug)]
pub struct Node<K: Ord, V> {
    key: K,
    pub val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Node<K, V> {
    pub const fn leaf(key: K, val: V) -> Self {
        Self {
            key,
            val,
            left: None,
            right: None,
        }
    }
    pub const fn key(&self) -> &K {
        &self.key
    }

    pub const fn kv(&self) -> (&K, &V) {
        (&self.key, &self.val)
    }

    pub fn children(&self) -> (Option<&Self>, Option<&Self>) {
        (self.left.as_deref(), self.right.as_deref())
    }

    pub fn search<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        use std::cmp::Ordering as O;
        match k.cmp(self.key.borrow()) {
            O::Less => self.left.as_ref()?.search(k),
            O::Equal => Some(&self.val),
            O::Greater => self.right.as_ref()?.search(k),
        }
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        use std::cmp::Ordering as O;
        match k.cmp(self.key.borrow()) {
            O::Less => match self.left {
                Some(ref mut x) => x.insert(k, v),
                None => {
                    self.left = Some(Box::new(Self::leaf(k, v)));
                    None
                }
            },
            O::Equal => Some(mem::replace(&mut self.val, v)),
            O::Greater => match self.right {
                Some(ref mut x) => x.insert(k, v),
                None => {
                    self.right = Some(Box::new(Self::leaf(k, v)));
                    None
                }
            },
        }
    }
}

impl<K: Ord, V> IntoIterator for Node<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(vec![self])
    }
}

pub struct IntoIter<K: Ord, V>(Vec<Node<K, V>>);

impl<K: Ord, V> IntoIter<K, V> {}

impl<K: Ord, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
        // Some(
        //     loop {
        //         let curr = self.0.pop()?;
        //         let Some(left) = curr.left else {
        //             if let Some(right) = curr.right {
        //                 self.0.push(*right)
        //             }
        //             break (curr.key, curr.val);
        //         };
        //         self.0.push(*left);
        //     }
        // )
    }
}
