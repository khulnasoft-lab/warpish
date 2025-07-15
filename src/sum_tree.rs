// A SumTree is a binary tree data structure where each node is the sum of its children.
// The values of the leaf nodes are the priorities (or weights) of the items.
// This implementation uses a flat array to represent the tree, similar to a binary heap.
// It is often used for prioritized sampling.

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct SumTree {
    nodes: Vec<f64>,
    capacity: usize,
}

impl SumTree {
    pub fn new(capacity: usize) -> Self {
        // The number of nodes in the tree is 2 * capacity - 1.
        // We use 2 * capacity to have 1-based indexing for easier parent/child calculations.
        let nodes = vec![0.0; 2 * capacity];
        Self { nodes, capacity }
    }

    fn update(&mut self, mut index: usize, priority: f64) {
        index += self.capacity;
        let change = priority - self.nodes[index];
        self.nodes[index] = priority;

        // Propagate the change up the tree
        while index > 1 {
            index /= 2;
            self.nodes[index] += change;
        }
    }

    pub fn set(&mut self, index: usize, priority: f64) {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }
        self.update(index, priority);
    }

    pub fn get(&self, mut value: f64) -> usize {
        let mut parent_index = 1;
        while parent_index < self.capacity {
            let left_child_index = parent_index * 2;
            let right_child_index = left_child_index + 1;

            if value < self.nodes[left_child_index] {
                parent_index = left_child_index;
            } else {
                value -= self.nodes[left_child_index];
                parent_index = right_child_index;
            }
        }
        parent_index - self.capacity
    }

    pub fn total(&self) -> f64 {
        self.nodes[1]
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Index<usize> for SumTree {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index + self.capacity]
    }
}

impl IndexMut<usize> for SumTree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }
        // We need to update the tree after mutation, but IndexMut doesn't allow that.
        // A `set` method is provided for safe updates.
        // For direct mutation, the user must call update manually.
        // This is a limitation of the IndexMut trait.
        // A better approach might be to return a proxy object that updates on drop.
        &mut self.nodes[index + self.capacity]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_tree_new() {
        let tree = SumTree::new(8);
        assert_eq!(tree.capacity(), 8);
        assert_eq!(tree.total(), 0.0);
    }

    #[test]
    fn test_sum_tree_set_and_total() {
        let mut tree = SumTree::new(4);
        tree.set(0, 10.0);
        tree.set(1, 5.0);
        tree.set(2, 15.0);
        tree.set(3, 20.0);

        assert_eq!(tree.total(), 50.0);
        assert_eq!(tree[0], 10.0);
        assert_eq!(tree[1], 5.0);
        assert_eq!(tree[2], 15.0);
        assert_eq!(tree[3], 20.0);
    }

    #[test]
    fn test_sum_tree_update() {
        let mut tree = SumTree::new(4);
        tree.set(0, 10.0);
        tree.set(1, 5.0);
        assert_eq!(tree.total(), 15.0);

        tree.set(0, 20.0);
        assert_eq!(tree.total(), 25.0);
        assert_eq!(tree[0], 20.0);
    }

    #[test]
    fn test_sum_tree_get() {
        let mut tree = SumTree::new(4);
        tree.set(0, 10.0); // Range: [0, 10)
        tree.set(1, 5.0);  // Range: [10, 15)
        tree.set(2, 15.0); // Range: [15, 30)
        tree.set(3, 20.0); // Range: [30, 50)

        assert_eq!(tree.get(0.0), 0);
        assert_eq!(tree.get(9.9), 0);
        assert_eq!(tree.get(10.0), 1);
        assert_eq!(tree.get(14.9), 1);
        assert_eq!(tree.get(15.0), 2);
        assert_eq!(tree.get(29.9), 2);
        assert_eq!(tree.get(30.0), 3);
        assert_eq!(tree.get(49.9), 3);
    }

    #[test]
    #[should_panic]
    fn test_sum_tree_index_out_of_bounds() {
        let mut tree = SumTree::new(4);
        tree.set(4, 10.0);
    }
}
