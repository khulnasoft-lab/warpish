//! Syntax Tree Module
//! 
//! This module provides Abstract Syntax Tree (AST) management capabilities
//! for various programming languages and structured data formats.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use tree_sitter::Tree;

pub mod node;
pub mod visitor;
pub mod builder;
pub mod analyzer;
pub mod transformer;

pub use node::*;
pub use visitor::*;
pub use builder::*;
pub use analyzer::*;
pub use transformer::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SyntaxError {
    ParseError(String),
    InvalidNode(String),
    VisitorError(String),
    TransformError(String),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SyntaxError::InvalidNode(msg) => write!(f, "Invalid node: {}", msg),
            SyntaxError::VisitorError(msg) => write!(f, "Visitor error: {}", msg),
            SyntaxError::TransformError(msg) => write!(f, "Transform error: {}", msg),
        }
    }
}

impl std::error::Error for SyntaxError {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Java,
    C,
    Cpp,
    Html,
    Css,
    Json,
    Yaml,
    Toml,
    Markdown,
    Generic(String),
}

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub tree: Tree,
    pub root: SyntaxNode,
}

// FIX: Added required associated types to trait definitions
pub trait Transformer {
    type Input;
    type Output;
    fn transform(&mut self, tree: &mut Self::Input) -> Self::Output;
}

pub trait Analyzer {
    type Input;
    type Output;
    fn analyze(&mut self, tree: &Self::Input) -> Self::Output;
}

impl SyntaxTree {
    pub fn transform<T: Transformer<Input = SyntaxTree>>(&mut self, transformer: &mut T) -> T::Output {
        transformer.transform(self)
    }

    pub fn analyze<A: Analyzer<Input = SyntaxTree>>(&self, analyzer: &mut A) -> A::Output {
        analyzer.analyze(self)
    }
    
    pub fn find_nodes_by_type(&self, node_type: &NodeType) -> Vec<&SyntaxNode> {
        let mut nodes = Vec::new();
        self.find_nodes_recursive(&self.root, node_type, &mut nodes);
        nodes
    }
    
    fn find_nodes_recursive<'a>(
        &'a self, 
        node: &'a SyntaxNode, 
        target_type: &NodeType, 
        results: &mut Vec<&'a SyntaxNode>
    ) {
        if &node.node_type == target_type {
            results.push(node);
        }
        
        for child in &node.children {
            self.find_nodes_recursive(child, target_type, results);
        }
    }
    
    pub fn pretty_print(&self) -> String {
        let mut result = String::new();
        self.pretty_print_recursive(&self.root, 0, &mut result);
        result
    }
    
    fn pretty_print_recursive(&self, node: &SyntaxNode, depth: usize, result: &mut String) {
        let indent = "  ".repeat(depth);
        result.push_str(&format!(
            "{}{:?} [{}..{}]\n",
            indent,
            node.node_type,
            node.span.start,
            node.span.end
        ));
        
        if let Some(ref value) = node.value {
            result.push_str(&format!("{}  value: {}\n", indent, value));
        }
        
        for child in &node.children {
            self.pretty_print_recursive(child, depth + 1, result);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub metrics: HashMap<String, f64>,
    pub issues: Vec<Issue>,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: IssueSeverity,
    pub message: String,
    pub span: Span,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub message: String,
    pub span: Span,
    pub replacement: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_tree_creation() {
        let root = SyntaxNode::new(
            NodeType::Root,
            Span::new(0, 10),
            None,
            Vec::new(),
        );
        
        let tree = SyntaxTree::new(
            root,
            Language::Rust,
            "fn main() {}".to_string(),
        );
        
        assert_eq!(tree.language, Language::Rust);
        assert_eq!(tree.source, "fn main() {}");
    }
    
    #[test]
    fn test_find_nodes_by_type() {
        let mut root = SyntaxNode::new(
            NodeType::Root,
            Span::new(0, 20),
            None,
            Vec::new(),
        );
        
        let function_node = SyntaxNode::new(
            NodeType::Function,
            Span::new(0, 10),
            Some("main".to_string()),
            Vec::new(),
        );
        
        root.children.push(function_node);
        
        let tree = SyntaxTree::new(
            root,
            Language::Rust,
            "fn main() {}".to_string(),
        );
        
        let function_nodes = tree.find_nodes_by_type(&NodeType::Function);
        assert_eq!(function_nodes.len(), 1);
        assert_eq!(function_nodes[0].value, Some("main".to_string()));
    }
    
    #[test]
    fn test_pretty_print() {
        let root = SyntaxNode::new(
            NodeType::Root,
            Span::new(0, 10),
            None,
            Vec::new(),
        );
        
        let tree = SyntaxTree::new(
            root,
            Language::Rust,
            "fn main() {}".to_string(),
        );
        
        let output = tree.pretty_print();
        assert!(output.contains("Root"));
        assert!(output.contains("[0..10]"));
    }
}
