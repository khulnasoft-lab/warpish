//! Virtual Filesystem Abstraction
//!
//! This module provides a virtual filesystem abstraction that can be used
//! to interact with different filesystems in a unified way.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// A trait for filesystem operations.
pub trait FileSystem {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    fn write(&mut self, path: &Path, data: &[u8]) -> io::Result<()>;
    fn list(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
}

/// An in-memory filesystem for testing and temporary storage.
#[derive(Default)]
pub struct InMemoryFileSystem {
    files: HashMap<PathBuf, Vec<u8>>,
}

impl InMemoryFileSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FileSystem for InMemoryFileSystem {
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn write(&mut self, path: &Path, data: &[u8]) -> io::Result<()> {
        self.files.insert(path.to_path_buf(), data.to_vec());
        Ok(())
    }

    fn list(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(self.files.keys().filter(|p| p.starts_with(path)).cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_file_system() {
        let mut fs = InMemoryFileSystem::new();
        let path = Path::new("/test.txt");
        let data = b"hello world";

        fs.write(path, data).unwrap();
        assert_eq!(fs.read(path).unwrap(), data);
        assert_eq!(fs.list(Path::new("/")).unwrap(), vec![path.to_path_buf()]);
    }
}
