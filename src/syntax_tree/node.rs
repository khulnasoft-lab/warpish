//! Syntax Tree Node Module
//! 
//! This module defines the core node structure for syntax trees.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }
    
    pub fn overlaps(&self, other: &Span) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Root,
    Function,
    Variable,
    Parameter,
    Statement,
    Expression,
    Block,
    Identifier,
    Literal,
    Comment,
    Keyword,
    Operator,
    Punctuation,
    Type,
    Import,
    Export,
    Class,
    Method,
    Property,
    If,
    While,
    For,
    Return,
    Try,
    Catch,
    Finally,
    Generic(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxNode {
    pub node_type: NodeType,
    pub span: Span,
    pub value: Option<String>,
    pub children: Vec<SyntaxNode>,
    pub attributes: HashMap<String, String>,
}

impl SyntaxNode {
    pub fn new(
        node_type: NodeType,
        span: Span,
        value: Option<String>,
        children: Vec<SyntaxNode>,
    ) -> Self {
        Self {
            node_type,
            span,
            value,
            children,
            attributes: HashMap::new(),
        }
    }
    
    pub fn with_attributes(mut self, attributes: HashMap<String, String>) -> Self {
        self.attributes = attributes;
        self
    }
    
    pub fn add_child(&mut self, child: SyntaxNode) {
        self.children.push(child);
    }
    
    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }
    
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }
    
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    
    pub fn find_child_by_type(&self, node_type: &NodeType) -> Option<&SyntaxNode> {
        self.children.iter().find(|child| &child.node_type == node_type)
    }
    
    pub fn find_children_by_type(&self, node_type: &NodeType) -> Vec<&SyntaxNode> {
        self.children.iter().filter(|child| &child.node_type == node_type).collect()
    }
    
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            0
        } else {
            1 + self.children.iter().map(|child| child.depth()).max().unwrap_or(0)
        }
    }
    
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|child| child.node_count()).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_creation() {
        let span = Span::new(5, 10);
        assert_eq!(span.len(), 5);
        assert!(!span.is_empty());
        assert!(span.contains(7));
        assert!(!span.contains(3));
    }
    
    #[test]
    fn test_span_overlaps() {
        let span1 = Span::new(5, 10);
        let span2 = Span::new(8, 15);
        assert!(span1.overlaps(&span2));
        
        let span3 = Span::new(15, 20);
        assert!(!span1.overlaps(&span3));
    }
    
    #[test]
    fn test_syntax_node_creation() {
        let node = SyntaxNode::new(
            NodeType::Function,
            Span::new(0, 10),
            Some("main".to_string()),
            Vec::new(),
        );
        
        assert_eq!(node.node_type, NodeType::Function);
        assert_eq!(node.value, Some("main".to_string()));
        assert!(node.is_leaf());
    }
    
    #[test]
    fn test_syntax_node_children() {
        let mut parent = SyntaxNode::new(
            NodeType::Function,
            Span::new(0, 20),
            Some("main".to_string()),
            Vec::new(),
        );
        
        let child = SyntaxNode::new(
            NodeType::Statement,
            Span::new(5, 15),
            None,
            Vec::new(),
        );
        
        parent.add_child(child);
        
        assert!(!parent.is_leaf());
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.node_count(), 2);
    }
    
    #[test]
    fn test_find_child_by_type() {
        let mut parent = SyntaxNode::new(
            NodeType::Function,
            Span::new(0, 20),
            Some("main".to_string()),
            Vec::new(),
        );
        
        let statement = SyntaxNode::new(
            NodeType::Statement,
            Span::new(5, 15),
            None,
            Vec::new(),
        );
        
        let expression = SyntaxNode::new(
            NodeType::Expression,
            Span::new(10, 12),
            None,
            Vec::new(),
        );
        
        parent.add_child(statement);
        parent.add_child(expression);
        
        assert!(parent.find_child_by_type(&NodeType::Statement).is_some());
        assert!(parent.find_child_by_type(&NodeType::Variable).is_none());
        
        let statements = parent.find_children_by_type(&NodeType::Statement);
        assert_eq!(statements.len(), 1);
    }
}
