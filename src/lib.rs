use std::{iter::Iterator, marker::PhantomData};

#[cfg(test)]
mod tests;

/// An element in the vector. It has the user value, a level starting from 0
/// for the root element, and a parent. The root's parent is 0.
pub struct Node<K, V> {
    /// The stored value
    pub value: V,
    /// Index of the parent node.
    parent: usize,
    /// The number of descendents, not including this node. This allows
    /// fast iteration of children.
    num_descendants: usize,
    /// This just exists because we didn't use K, but we want it to be part
    /// of the type.
    _key_type: PhantomData<K>,
}

impl<K, V> Node<K, V>
where
    usize: Into<K>,
{
    /// Get the ID of the parent node. The ID of the root node is equal to
    /// the root ID (so check for loops!).
    pub fn parent(&self) -> K {
        self.parent.into()
    }

    /// The number of descendents, not including this node.
    pub fn num_descendants(&self) -> usize {
        self.num_descendants
    }
}

/// A flattened tree. The nodes are stored in pre-order (depth first order).
pub struct Tree<K, V> {
    nodes: Vec<Node<K, V>>,
    parent_stack: Vec<usize>,
}

impl<K, V> Default for Tree<K, V> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            parent_stack: Default::default(),
        }
    }
}

impl<K, V> Tree<K, V>
where
    usize: Into<K>,
    K: Into<usize>,
{
    /// Create a new empty tree containing no nodes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an empty tree with reserved capacity for a certain number of nodes.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            parent_stack: Vec::new(),
        }
    }

    /// Return the total number of nodes in the tree.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Return true if the tree is empty. In this case it has no root node.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Push a child of the current node. If this is the first node it will
    /// become the root. The new node becomes the current node.
    ///
    /// # WARNING
    ///
    /// If you don't push with the correct values then iteration may give
    /// unexpected results.
    pub fn push(&mut self, value: V) -> K {
        let id = self.len();

        self.nodes.push(Node {
            value,
            parent: *self.parent_stack.last().unwrap_or(&id),
            num_descendants: 0,
            _key_type: PhantomData,
        });

        // Increment the descendent counts of all parent nodes.
        for &parent in self.parent_stack.iter() {
            self.nodes[parent].num_descendants += 1;
        }

        self.parent_stack.push(id);

        id.into()
    }

    /// Set the current node to its parent. It's safe to call this if the
    /// the tree is empty, in which case nothign will change.
    ///
    /// It is ok to call this if the current node is the root node. If you then
    /// add more nodes you will end up with a tree with multiple root nodes.
    /// this should work fine but might be confusing!
    pub fn up(&mut self) {
        self.parent_stack.pop();
    }

    /// Get a reference to a node. Returns `None` for invalid IDs.
    pub fn get(&self, id: K) -> Option<&Node<K, V>> {
        self.nodes.get(id.into())
    }

    /// Get a mutable reference to a node. Returns `None` for invalid IDs.
    pub fn get_mut(&mut self, id: K) -> Option<&mut Node<K, V>> {
        self.nodes.get_mut(id.into())
    }

    /// Get a reference to the root node (or `None` if the tree is empty).
    /// It is possible to have a tree with multiple roots, but this only
    /// returns the first one.
    pub fn root(&self) -> Option<&Node<K, V>> {
        self.nodes.get(0)
    }

    /// Get a mutable reference to the root node (or `None` if the tree is empty).
    /// It is possible to have a tree with multiple roots, but this only
    /// returns the first one.
    pub fn root_mut(&mut self) -> Option<&mut Node<K, V>> {
        self.nodes.get_mut(0)
    }

    /// Iterate through all the tree nodes in the order they were added (which
    /// must be pre-order / depth first).
    pub fn iter(&self) -> impl Iterator<Item = &Node<K, V>> {
        self.nodes.iter()
    }

    /// Convert the tree into an iterator through all the tree nodes in the
    /// order they were added (which must be pre-order / depth first).
    pub fn into_iter(self) -> impl Iterator<Item = Node<K, V>> {
        self.nodes.into_iter()
    }

    /// Get a slice of all the descendents of a node.
    pub fn descendents(&self, id: K) -> &[Node<K, V>] {
        let id = id.into();
        let num_descendants = self
            .nodes
            .get(id)
            .map(|node| node.num_descendants)
            .unwrap_or_default();
        &self.nodes[id + 1..id + 1 + num_descendants]
    }

    /// Get an iterator over the parents of a node (not including the node itself).
    pub fn parents(&self, id: K) -> ParentIter<'_, K, V> {
        let id = id.into();
        ParentIter { id, tree: self }
    }

    /// Get an iterator over the immediate children of a node.
    pub fn children(&self, id: K) -> ChildrenIter<'_, K, V> {
        let id = id.into();
        ChildrenIter {
            current_id: id + 1,
            max_id: id
                + self
                    .nodes
                    .get(id)
                    .map(|node| node.num_descendants)
                    .unwrap_or_default(),
            tree: self,
        }
    }
}

pub struct ParentIter<'a, K, V> {
    id: usize,
    tree: &'a Tree<K, V>,
}

impl<'a, K, V> Iterator for ParentIter<'a, K, V>
where
    K: Into<usize>,
    usize: Into<K>,
{
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tree.nodes.get(self.id).and_then(|node| {
            if node.parent == self.id {
                None
            } else {
                self.id = node.parent;
                self.tree.nodes.get(self.id)
            }
        })
    }
}

pub struct ChildrenIter<'a, K, V> {
    current_id: usize,
    max_id: usize,
    tree: &'a Tree<K, V>,
}

impl<'a, K, V> Iterator for ChildrenIter<'a, K, V> {
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id <= self.max_id {
            let node = self.tree.nodes.get(self.current_id);
            self.current_id += self
                .tree
                .nodes
                .get(self.current_id)
                .map(|node| node.num_descendants)
                .unwrap_or_default()
                + 1;
            node
        } else {
            None
        }
    }
}
