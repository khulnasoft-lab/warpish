use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    Uninitialized,
    DiscoveringFiles,
    Synced,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Codebase {
    pub path: PathBuf,
    pub file_count: usize,
    #[serde(default)]
    pub status: SyncStatus,
}

impl Default for SyncStatus {
    fn default() -> Self { SyncStatus::Uninitialized }
}

/// The manager that handles all indexed codebases.
#[derive(Debug, Clone)]
pub struct CodebaseManager {
    // We use a Mutex to allow mutation from a background thread.
    pub repositories: Arc<Mutex<HashMap<PathBuf, Codebase>>>,
    // Sender to notify the main thread that state has changed.
    update_tx: mpsc::Sender<()>,
}

impl CodebaseManager {
    pub fn new(update_tx: mpsc::Sender<()>) -> Self {
        Self {
            repositories: Arc::new(Mutex::new(HashMap::new())),
            update_tx,
        }
    }

    /// Spawns a background task to index a given repository path.
    pub fn index_repository(&self, path: &Path) {
        let git_root = match find_git_root(path) {
            Some(root) => root,
            None => {
                log::warn!("'{}' is not in a git repository. Cannot index.", path.display());
                return;
            }
        };

        log::info!("Starting to index repository at: {}", git_root.display());

        let repos_clone = self.repositories.clone();
        let update_tx_clone = self.update_tx.clone();

        tokio::spawn(async move {
            {
                // Set status to DiscoveringFiles immediately
                let mut repos = repos_clone.lock().unwrap();
                let entry = repos.entry(git_root.clone()).or_insert_with(|| Codebase {
                    path: git_root.clone(),
                    file_count: 0,
                    status: SyncStatus::Uninitialized,
                });
                entry.status = SyncStatus::DiscoveringFiles;
            }
            // Notify UI of status change
            update_tx_clone.send(()).await.ok();
            
            // Simulate work
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            // Use the `ignore` crate to respect .gitignore and other files
            let walker = WalkBuilder::new(&git_root).build();
            let file_count = walker.filter_map(Result::ok).filter(|e| e.path().is_file()).count();
            
            // In a real implementation, this is where you'd generate embeddings.
            
            // Update the final state
            let mut repos = repos_clone.lock().unwrap();
            let entry = repos.get_mut(&git_root).unwrap();
            entry.file_count = file_count;
            entry.status = SyncStatus::Synced; // Or Failed if something went wrong

            log::info!("Finished indexing {}. Found {} files.", git_root.display(), file_count);
            // Notify UI that indexing is complete
            update_tx_clone.send(()).await.ok();
        });
    }
}

fn find_git_root(start_path: &Path) -> Option<PathBuf> {
    let mut current = start_path.to_path_buf();
    loop {
        if current.join(".git").is_dir() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
} 