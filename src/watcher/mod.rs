//! File System Monitoring and Change Detection
//!
//! This module provides file system monitoring and change detection using the `notify` crate.

use notify::{RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatcherError {
    #[error("Failed to create watcher: {0}")]
    CreateFailed(notify::Error),
    #[error("Failed to watch path: {0}")]
    WatchFailed(notify::Error),
}

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<notify::Event>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self, WatcherError> {
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(tx, notify::Config::default()).map_err(WatcherError::CreateFailed)?;
        Ok(Self { watcher, rx })
    }

    pub fn watch(&mut self, path: &Path) -> Result<(), WatcherError> {
        self.watcher
            .watch(path, RecursiveMode::Recursive)
            .map_err(WatcherError::WatchFailed)
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(path)
    }

    pub fn events(&self) -> &Receiver<notify::Result<notify::Event>> {
        &self.rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_file_watcher() {
        let mut watcher = FileWatcher::new().unwrap();
        let dir = tempfile::tempdir().unwrap();
        watcher.watch(dir.path()).unwrap();

        let file_path = dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "hello").unwrap();

        // Check for the event
        let event = watcher.events().recv().unwrap().unwrap();
        assert_eq!(event.paths, vec![file_path]);
    }
}
