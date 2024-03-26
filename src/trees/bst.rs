pub struct Node<K: Ord, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

pub type BST<K, V> = Node<K, V>;
