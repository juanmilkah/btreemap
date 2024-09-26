/*A BTreeMap implementation
 * the data structure is commonly used in database building
 * to optomize query searching nistead of a normal HashMap as used in std::collections::HashMap;
 * BTreeMap store the key Values in sorted order
 *For more reference check out std:collections::BTreeMap;
 * September 25, 2024
 */

#[derive(Debug, Clone)]
struct BTreeNode<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<BTreeNode<K, V>>>,
    is_leaf: bool,
    min_degree: usize,
}

impl<K: Ord + Clone, V: Clone> BTreeNode<K, V> {
    fn new(min_degree: usize, is_leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf,
            min_degree,
        }
    }

    fn insert_non_full(&mut self, key: K, value: V) {
        let pos = self
            .keys
            .iter()
            .position(|k| *k >= key)
            .unwrap_or(self.keys.len());

        if self.is_leaf {
            self.keys.insert(pos, key);
            self.values.insert(pos, value);
        } else {
            if self.children[pos].keys.len() == 2 * self.min_degree - 1 {
                self.split_child(pos);
                if key > self.keys[pos] {
                    self.children[pos + 1].insert_non_full(key, value);
                } else {
                    self.children[pos].insert_non_full(key, value);
                }
            } else {
                self.children[pos].insert_non_full(key, value);
            }
        }
    }

    fn split_child(&mut self, index: usize) {
        let min_degree = self.min_degree;
        let mut new_child = Box::new(BTreeNode::new(min_degree, self.children[index].is_leaf));

        let child = &mut self.children[index];
        new_child.keys.extend(child.keys.split_off(min_degree));
        new_child.values.extend(child.values.split_off(min_degree));
        if !child.is_leaf {
            new_child
                .children
                .extend(child.children.split_off(min_degree));
        }

        self.keys.insert(index, child.keys.pop().unwrap());
        self.values.insert(index, child.values.pop().unwrap());
        self.children.insert(index + 1, new_child);
    }
}

#[derive(Debug)]
struct BTreeMap<K, V> {
    root: Option<BTreeNode<K, V>>,
    min_degree: usize,
}

impl<K: Ord + Clone, V: Clone> BTreeMap<K, V> {
    fn new(min_degree: usize) -> Self {
        BTreeMap {
            root: None,
            min_degree,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            self.root = Some(BTreeNode::new(self.min_degree, true));
        }

        if let Some(ref mut root) = self.root {
            if root.keys.len() == 2 * self.min_degree - 1 {
                let mut new_root = BTreeNode::new(self.min_degree, false);
                new_root.children.push(Box::new(root.clone()));
                new_root.split_child(0);
                new_root.insert_non_full(key, value);
                self.root = Some(new_root);
            } else {
                root.insert_non_full(key, value);
            }
        }
    }

    fn search(&self, key: &K) -> Option<&V> {
        self.root
            .as_ref()
            .and_then(|node| self.search_in_node(node, key))
    }

    /*use recursion to search the node tree*/
    fn search_in_node<'a>(&self, node: &'a BTreeNode<K, V>, key: &K) -> Option<&'a V> {
        let pos = node
            .keys
            .iter()
            .position(|k| k >= key)
            .unwrap_or(node.keys.len());

        if pos < node.keys.len() && &node.keys[pos] == key {
            return Some(&node.values[pos]);
        }

        /*if we get to this point  and the node is leaf, then key does not exist in the tree*/
        if node.is_leaf {
            None
        } else {
            node.children
                .get(pos)
                .and_then(|child| self.search_in_node(child, key))
        }
    }
}

fn main() {
    /*test samples*/
    let mut btree = BTreeMap::new(2); // Minimum degree of 2
    btree.insert(10, "Ten");
    btree.insert(20, "Twenty");
    btree.insert(5, "Five");
    btree.insert(6, "Six");
    btree.insert(12, "Twelve");

    if let Some(value) = btree.search(&10) {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }

    if let Some(value) = btree.search(&7) {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }
}
