//! Blocks Module
//! 
//! This module implements Warp-inspired Blocks functionality for grouping commands
//! and outputs into atomic units for better terminal interaction.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub command: String,
    pub output: String,
    pub status: CommandStatus,
    pub timestamp: u64,
    pub execution_time: Option<Duration>,
    pub working_directory: String,
    pub environment: HashMap<String, String>,
    pub bookmarked: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandStatus {
    Running,
    Success,
    Error(i32), // Exit code
    Cancelled,
}

impl Block {
    pub fn new(command: String, working_directory: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            command,
            output: String::new(),
            status: CommandStatus::Running,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            execution_time: None,
            working_directory,
            environment: HashMap::new(),
            bookmarked: false,
            tags: Vec::new(),
        }
    }

    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }

    pub fn set_status(&mut self, status: CommandStatus) {
        self.status = status;
    }

    pub fn set_execution_time(&mut self, duration: Duration) {
        self.execution_time = Some(duration);
    }

    pub fn toggle_bookmark(&mut self) {
        self.bookmarked = !self.bookmarked;
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn copy_command(&self) -> String {
        self.command.clone()
    }

    pub fn copy_output(&self) -> String {
        self.output.clone()
    }

    pub fn copy_both(&self) -> String {
        format!("$ {}\n{}", self.command, self.output)
    }

    pub fn is_successful(&self) -> bool {
        matches!(self.status, CommandStatus::Success)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status, CommandStatus::Running)
    }

    pub fn format_for_sharing(&self) -> String {
        let status_symbol = match self.status {
            CommandStatus::Success => "✅",
            CommandStatus::Error(_) => "❌",
            CommandStatus::Running => "⏳",
            CommandStatus::Cancelled => "⏹️",
        };

        format!(
            "{} Command: {}\nOutput:\n{}\nDirectory: {}\nTime: {}",
            status_symbol,
            self.command,
            self.output,
            self.working_directory,
            self.format_timestamp()
        )
    }

    fn format_timestamp(&self) -> String {
        // Simple timestamp formatting - in real implementation, use chrono
        format!("timestamp: {}", self.timestamp)
    }
}

#[derive(Debug, Clone)]
pub struct BlockManager {
    blocks: Vec<Block>,
    current_block_index: Option<usize>,
    max_blocks: usize,
}

impl BlockManager {
    pub fn new(max_blocks: usize) -> Self {
        Self {
            blocks: Vec::new(),
            current_block_index: None,
            max_blocks,
        }
    }

    pub fn create_block(&mut self, command: String, working_directory: String) -> &mut Block {
        let block = Block::new(command, working_directory);
        self.blocks.push(block);
        
        // Keep only the latest max_blocks
        if self.blocks.len() > self.max_blocks {
            self.blocks.remove(0);
        }
        
        self.current_block_index = Some(self.blocks.len() - 1);
        self.blocks.last_mut().unwrap()
    }

    pub fn get_current_block(&mut self) -> Option<&mut Block> {
        if let Some(index) = self.current_block_index {
            self.blocks.get_mut(index)
        } else {
            None
        }
    }

    pub fn get_block_by_id(&self, id: &str) -> Option<&Block> {
        self.blocks.iter().find(|b| b.id == id)
    }

    pub fn get_block_by_id_mut(&mut self, id: &str) -> Option<&mut Block> {
        self.blocks.iter_mut().find(|b| b.id == id)
    }

    pub fn get_all_blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn get_bookmarked_blocks(&self) -> Vec<&Block> {
        self.blocks.iter().filter(|b| b.bookmarked).collect()
    }

    pub fn get_blocks_by_tag(&self, tag: &str) -> Vec<&Block> {
        self.blocks.iter().filter(|b| b.tags.contains(&tag.to_string())).collect()
    }

    pub fn search_blocks(&self, query: &str) -> Vec<&Block> {
        let query = query.to_lowercase();
        self.blocks
            .iter()
            .filter(|b| {
                b.command.to_lowercase().contains(&query) 
                || b.output.to_lowercase().contains(&query)
                || b.working_directory.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn navigate_to_block(&mut self, id: &str) -> Option<usize> {
        if let Some(index) = self.blocks.iter().position(|b| b.id == id) {
            self.current_block_index = Some(index);
            Some(index)
        } else {
            None
        }
    }

    pub fn get_previous_block(&mut self) -> Option<&Block> {
        if let Some(current) = self.current_block_index {
            if current > 0 {
                self.current_block_index = Some(current - 1);
                self.blocks.get(current - 1)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_next_block(&mut self) -> Option<&Block> {
        if let Some(current) = self.current_block_index {
            if current < self.blocks.len() - 1 {
                self.current_block_index = Some(current + 1);
                self.blocks.get(current + 1)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn delete_block(&mut self, id: &str) -> bool {
        if let Some(index) = self.blocks.iter().position(|b| b.id == id) {
            self.blocks.remove(index);
            
            // Adjust current_block_index if necessary
            if let Some(current) = self.current_block_index {
                if current >= index {
                    self.current_block_index = if current > 0 { Some(current - 1) } else { None };
                }
            }
            
            true
        } else {
            false
        }
    }

    pub fn clear_all_blocks(&mut self) {
        self.blocks.clear();
        self.current_block_index = None;
    }

    pub fn get_statistics(&self) -> BlockStatistics {
        let total_blocks = self.blocks.len();
        let successful_blocks = self.blocks.iter().filter(|b| b.is_successful()).count();
        let bookmarked_blocks = self.blocks.iter().filter(|b| b.bookmarked).count();
        let running_blocks = self.blocks.iter().filter(|b| b.is_running()).count();

        let avg_execution_time = if total_blocks > 0 {
            let total_time: Duration = self.blocks
                .iter()
                .filter_map(|b| b.execution_time)
                .sum();
            Some(total_time / total_blocks as u32)
        } else {
            None
        };

        BlockStatistics {
            total_blocks,
            successful_blocks,
            bookmarked_blocks,
            running_blocks,
            avg_execution_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatistics {
    pub total_blocks: usize,
    pub successful_blocks: usize,
    pub bookmarked_blocks: usize,
    pub running_blocks: usize,
    pub avg_execution_time: Option<Duration>,
}

impl Default for BlockManager {
    fn default() -> Self {
        Self::new(1000) // Default to keeping 1000 blocks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new("ls -la".to_string(), "/home/user".to_string());
        assert_eq!(block.command, "ls -la");
        assert_eq!(block.working_directory, "/home/user");
        assert!(matches!(block.status, CommandStatus::Running));
        assert!(!block.bookmarked);
        assert!(block.tags.is_empty());
    }

    #[test]
    fn test_block_operations() {
        let mut block = Block::new("echo hello".to_string(), "/tmp".to_string());
        
        block.set_output("hello\n".to_string());
        assert_eq!(block.output, "hello\n");
        
        block.set_status(CommandStatus::Success);
        assert!(block.is_successful());
        
        block.toggle_bookmark();
        assert!(block.bookmarked);
        
        block.add_tag("important".to_string());
        assert!(block.tags.contains(&"important".to_string()));
    }

    #[test]
    fn test_block_manager() {
        let mut manager = BlockManager::new(3);
        
        // Create blocks
        let block1 = manager.create_block("ls".to_string(), "/home".to_string());
        let block1_id = block1.id.clone();
        
        let block2 = manager.create_block("pwd".to_string(), "/home".to_string());
        let block2_id = block2.id.clone();
        
        assert_eq!(manager.get_all_blocks().len(), 2);
        
        // Test navigation
        assert!(manager.navigate_to_block(&block1_id).is_some());
        assert!(manager.navigate_to_block("nonexistent").is_none());
        
        // Test search
        let search_results = manager.search_blocks("ls");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].command, "ls");
        
        // Test bookmarking
        if let Some(block) = manager.get_block_by_id_mut(&block1_id) {
            block.toggle_bookmark();
        }
        
        let bookmarked = manager.get_bookmarked_blocks();
        assert_eq!(bookmarked.len(), 1);
        
        // Test statistics
        let stats = manager.get_statistics();
        assert_eq!(stats.total_blocks, 2);
        assert_eq!(stats.bookmarked_blocks, 1);
    }

    #[test]
    fn test_block_max_limit() {
        let mut manager = BlockManager::new(2);
        
        manager.create_block("cmd1".to_string(), "/".to_string());
        manager.create_block("cmd2".to_string(), "/".to_string());
        manager.create_block("cmd3".to_string(), "/".to_string());
        
        // Should only keep the latest 2 blocks
        assert_eq!(manager.get_all_blocks().len(), 2);
        
        let blocks = manager.get_all_blocks();
        assert_eq!(blocks[0].command, "cmd2");
        assert_eq!(blocks[1].command, "cmd3");
    }

    #[test]
    fn test_block_sharing_format() {
        let mut block = Block::new("echo 'Hello World'".to_string(), "/tmp".to_string());
        block.set_output("Hello World\n".to_string());
        block.set_status(CommandStatus::Success);
        
        let shared = block.format_for_sharing();
        assert!(shared.contains("✅"));
        assert!(shared.contains("echo 'Hello World'"));
        assert!(shared.contains("Hello World"));
        assert!(shared.contains("/tmp"));
    }
}
