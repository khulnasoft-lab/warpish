//! String Offset Module
//! 
//! This module provides utilities for efficient string manipulation,
//! including operations like substring extraction, search, replace, and more.

use std::ops::{Range, RangeBounds};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringOffset {
    pub start: usize,
    pub end: usize,
}

impl StringOffset {
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
    
    pub fn overlaps(&self, other: &StringOffset) -> bool {
        self.start < other.end && other.start < self.end
    }
    
    pub fn merge(&self, other: &StringOffset) -> Option<StringOffset> {
        if self.overlaps(other) || self.end == other.start || other.end == self.start {
            Some(StringOffset::new(
                self.start.min(other.start),
                self.end.max(other.end),
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringManipulator {
    content: String,
    offsets: Vec<StringOffset>,
}

impl StringManipulator {
    pub fn new(content: String) -> Self {
        Self {
            content,
            offsets: Vec::new(),
        }
    }
    
    pub fn substring(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&bound) => bound,
            std::ops::Bound::Excluded(&bound) => bound + 1,
            std::ops::Bound::Unbounded => 0,
        };
        
        let end = match range.end_bound() {
            std::ops::Bound::Included(&bound) => bound + 1,
            std::ops::Bound::Excluded(&bound) => bound,
            std::ops::Bound::Unbounded => self.content.len(),
        };
        
        &self.content[start..end]
    }
    
    pub fn find_all(&self, pattern: &str) -> Vec<StringOffset> {
        let mut offsets = Vec::new();
        let mut start = 0;
        
        while let Some(pos) = self.content[start..].find(pattern) {
            let absolute_pos = start + pos;
            offsets.push(StringOffset::new(absolute_pos, absolute_pos + pattern.len()));
            start = absolute_pos + pattern.len();
        }
        
        offsets
    }
    
    pub fn replace_all(&mut self, from: &str, to: &str) {
        self.content = self.content.replace(from, to);
    }
    
    pub fn insert_at(&mut self, offset: usize, text: &str) {
        if offset <= self.content.len() {
            self.content.insert_str(offset, text);
        }
    }
    
    pub fn delete_range(&mut self, range: Range<usize>) {
        if range.start <= self.content.len() && range.end <= self.content.len() {
            self.content.drain(range);
        }
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
    
    pub fn len(&self) -> usize {
        self.content.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    
    pub fn lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }
    
    pub fn line_offsets(&self) -> Vec<StringOffset> {
        let mut offsets = Vec::new();
        let mut start = 0;
        
        for line in self.content.lines() {
            let end = start + line.len();
            offsets.push(StringOffset::new(start, end));
            start = end + 1; // +1 for newline character
        }
        
        offsets
    }
    
    pub fn word_offsets(&self) -> Vec<StringOffset> {
        let mut offsets = Vec::new();
        let mut chars = self.content.char_indices().peekable();
        
        while let Some((start, ch)) = chars.next() {
            if ch.is_alphabetic() {
                let mut end = start + ch.len_utf8();
                
                while let Some(&(next_start, next_ch)) = chars.peek() {
                    if next_ch.is_alphabetic() || next_ch == '_' {
                        end = next_start + next_ch.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                offsets.push(StringOffset::new(start, end));
            }
        }
        
        offsets
    }
    
    pub fn validate_utf8(&self) -> bool {
        std::str::from_utf8(self.content.as_bytes()).is_ok()
    }
    
    pub fn char_count(&self) -> usize {
        self.content.chars().count()
    }
    
    pub fn byte_count(&self) -> usize {
        self.content.len()
    }
}

// Convenience functions
pub fn substring(s: &str, range: impl RangeBounds<usize>) -> &str {
    let start = match range.start_bound() {
        std::ops::Bound::Included(&bound) => bound,
        std::ops::Bound::Excluded(&bound) => bound + 1,
        std::ops::Bound::Unbounded => 0,
    };
    
    let end = match range.end_bound() {
        std::ops::Bound::Included(&bound) => bound + 1,
        std::ops::Bound::Excluded(&bound) => bound,
        std::ops::Bound::Unbounded => s.len(),
    };
    
    &s[start..end]
}

pub fn find_substring(s: &str, pattern: &str) -> Option<usize> {
    s.find(pattern)
}

pub fn replace_substring(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn split_string(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
}

pub fn validate_utf8(s: &str) -> bool {
    std::str::from_utf8(s.as_bytes()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_offset() {
        let offset = StringOffset::new(5, 10);
        assert_eq!(offset.len(), 5);
        assert!(!offset.is_empty());
        assert!(offset.contains(7));
        assert!(!offset.contains(3));
    }
    
    #[test]
    fn test_string_manipulator() {
        let mut manipulator = StringManipulator::new("Hello, World!".to_string());
        
        assert_eq!(manipulator.substring(0..5), "Hello");
        assert_eq!(manipulator.len(), 13);
        
        let offsets = manipulator.find_all("l");
        assert_eq!(offsets.len(), 3);
        
        manipulator.replace_all("World", "Rust");
        assert_eq!(manipulator.content(), "Hello, Rust!");
    }
    
    #[test]
    fn test_line_offsets() {
        let manipulator = StringManipulator::new("Line 1\nLine 2\nLine 3".to_string());
        let line_offsets = manipulator.line_offsets();
        
        assert_eq!(line_offsets.len(), 3);
        assert_eq!(line_offsets[0], StringOffset::new(0, 6));
        assert_eq!(line_offsets[1], StringOffset::new(7, 13));
        assert_eq!(line_offsets[2], StringOffset::new(14, 20));
    }
    
    #[test]
    fn test_word_offsets() {
        let manipulator = StringManipulator::new("Hello world test".to_string());
        let word_offsets = manipulator.word_offsets();
        
        assert_eq!(word_offsets.len(), 3);
        assert_eq!(word_offsets[0], StringOffset::new(0, 5)); // "Hello"
        assert_eq!(word_offsets[1], StringOffset::new(6, 11)); // "world"
        assert_eq!(word_offsets[2], StringOffset::new(12, 16)); // "test"
    }
    
    #[test]
    fn test_offset_merge() {
        let offset1 = StringOffset::new(0, 5);
        let offset2 = StringOffset::new(3, 8);
        let merged = offset1.merge(&offset2).unwrap();
        
        assert_eq!(merged, StringOffset::new(0, 8));
    }
    
    #[test]
    fn test_convenience_functions() {
        assert_eq!(substring("Hello, World!", 0..5), "Hello");
        assert_eq!(find_substring("Hello, World!", "World"), Some(7));
        assert_eq!(replace_substring("Hello, World!", "World", "Rust"), "Hello, Rust!");
        
        let parts = split_string("one,two,three", ',');
        assert_eq!(parts, vec!["one", "two", "three"]);
        
        assert!(validate_utf8("Valid UTF-8 text"));
    }
}
