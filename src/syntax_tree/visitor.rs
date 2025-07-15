//! Syntax Tree Visitor Module
//! 
//! This module provides the visitor pattern implementation for syntax trees.

use super::{SyntaxError, SyntaxNode, SyntaxTree};

pub trait Visitor {
    fn visit_tree(&mut self, tree: &SyntaxTree) -> Result<(), SyntaxError> {
        self.visit_node(&tree.root)
    }
    
    fn visit_node(&mut self, node: &SyntaxNode) -> Result<(), SyntaxError> {
        self.enter_node(node)?;
        
        for child in &node.children {
            self.visit_node(child)?;
        }
        
        self.exit_node(node)?;
        Ok(())
    }
    
    fn enter_node(&mut self, node: &SyntaxNode) -> Result<(), SyntaxError>;
    fn exit_node(&mut self, node: &SyntaxNode) -> Result<(), SyntaxError>;
}

pub struct PrintVisitor {
    depth: usize,
    output: String,
}

impl PrintVisitor {
    pub fn new() -> Self {
        Self {
            depth: 0,
            output: String::new(),
        }
    }
    
    pub fn output(&self) -> &str {
        &self.output
    }
}

impl Visitor for PrintVisitor {
    fn enter_node(&mut self, node: &SyntaxNode) -> Result<(), SyntaxError> {
        let indent = "  ".repeat(self.depth);
        self.output.push_str(&format!(
            "{}{:?} [{}..{}]",
            indent,
            node.node_type,
            node.span.start,
            node.span.end
        ));
        
        if let Some(ref value) = node.value {
            self.output.push_str(&format!(" = {}", value));
        }
        
        self.output.push('\n');
        self.depth += 1;
        Ok(())
    }
    
    fn exit_node(&mut self, _node: &SyntaxNode) -> Result<(), SyntaxError> {
        self.depth -= 1;
        Ok(())
    }
}

pub struct CountVisitor {
    node_counts: std::collections::HashMap<String, usize>,
}

impl CountVisitor {
    pub fn new() -> Self {
        Self {
            node_counts: std::collections::HashMap::new(),
        }
    }
    
    pub fn counts(&self) -> &std::collections::HashMap<String, usize> {
        &self.node_counts
    }
    
    pub fn total_nodes(&self) -> usize {
        self.node_counts.values().sum()
    }
}

impl Visitor for CountVisitor {
    fn enter_node(&mut self, node: &SyntaxNode) -> Result<(), SyntaxError> {
        let node_type = format!("{:?}", node.node_type);
        *self.node_counts.entry(node_type).or_insert(0) += 1;
        Ok(())
    }
    
    fn exit_node(&mut self, _node: &SyntaxNode) -> Result<(), SyntaxError> {
        Ok(())
    }
}

pub struct ValidationVisitor {
    errors: Vec<String>,
}

impl ValidationVisitor {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }
    
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Visitor for ValidationVisitor {
    fn enter_node(&mut self, node: &SyntaxNode) -> Result<(), SyntaxError> {
        // Basic validation rules
        if node.span.start > node.span.end {
            self.errors.push(format!(
                "Invalid span: start ({}) > end ({})",
                node.span.start,
                node.span.end
            ));
        }
        
        // Check that children are within parent span
        for child in &node.children {
            if child.span.start < node.span.start || child.span.end > node.span.end {
                self.errors.push(format!(
                    "Child span [{}, {}] extends beyond parent span [{}, {}]",
                    child.span.start,
                    child.span.end,
                    node.span.start,
                    node.span.end
                ));
            }
        }
        
        Ok(())
    }
    
    fn exit_node(&mut self, _node: &SyntaxNode) -> Result<(), SyntaxError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax_tree::{Language, NodeType, Span};

    #[test]
    fn test_print_visitor() {
        let mut root = SyntaxNode::new(
            NodeType::Root,
            Span::new(0, 20),
            None,
            Vec::new(),
        );
        
        let function = SyntaxNode::new(
            NodeType::Function,
            Span::new(0, 10),
            Some("main".to_string()),
            Vec::new(),
        );
        
        root.add_child(function);
        
        let tree = SyntaxTree::new(root, Language::Rust, "fn main() {}".to_string());
        
        let mut visitor = PrintVisitor::new();
        tree.accept(&mut visitor).unwrap();
        
        let output = visitor.output();
        assert!(output.contains("Root"));
        assert!(output.contains("Function"));
        assert!(output.contains("main"));
    }
    
    #[test]
    fn test_count_visitor() {
        let mut root = SyntaxNode::new(
            NodeType::Root,
            Span::new(0, 20),
            None,
            Vec::new(),
        );
        
        let function = SyntaxNode::new(
            NodeType::Function,
            Span::new(0, 10),
            Some("main".to_string()),
            Vec::new(),
        );
        
        root.add_child(function);
        
        let tree = SyntaxTree::new(root, Language::Rust, "fn main() {}".to_string());
        
        let mut visitor = CountVisitor::new();
        tree.accept(&mut visitor).unwrap();
        
        assert_eq!(visitor.total_nodes(), 2);
        assert_eq!(visitor.counts().get("Root"), Some(&1));
        assert_eq!(visitor.counts().get("Function"), Some(&1));
    }
    
    #[test]
    fn test_validation_visitor() {
        let mut root = SyntaxNode::new(
            NodeType::Root,
            Span::new(0, 20),
            None,
            Vec::new(),
        );
        
        // Add a child with invalid span (extends beyond parent)
        let invalid_child = SyntaxNode::new(
            NodeType::Function,
            Span::new(0, 25), // extends beyond parent's end (20)
            Some("main".to_string()),
            Vec::new(),
        );
        
        root.add_child(invalid_child);
        
        let tree = SyntaxTree::new(root, Language::Rust, "fn main() {}".to_string());
        
        let mut visitor = ValidationVisitor::new();
        tree.accept(&mut visitor).unwrap();
        
        assert!(!visitor.is_valid());
        assert_eq!(visitor.errors().len(), 1);
        assert!(visitor.errors()[0].contains("extends beyond parent span"));
    }
}
