#[derive(Debug, Clone)]
pub struct SumTreeNode<T> {
    pub value: T,
    pub children: Vec<SumTreeNode<T>>,
}

impl<T> SumTreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: SumTreeNode<T>) {
        self.children.push(child);
    }

    pub fn depth_first(&self, results: &mut Vec<&T>) {
        results.push(&self.value);
        for child in &self.children {
            child.depth_first(results);
        }
    }

    pub fn map<F, U>(&self, f: &F) -> SumTreeNode<U>
    where
        F: Fn(&T) -> U,
    {
        SumTreeNode {
            value: f(&self.value),
            children: self.children.iter().map(|c| c.map(f)).collect(),
        }
    }
}