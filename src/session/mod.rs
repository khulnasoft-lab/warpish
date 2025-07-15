//! Session management
//!
//! This module handles session creation, restoration, and management.

pub mod sqlite;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub tabs: Vec<String>,
}

impl Session {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            tabs: vec!["default".to_string()],
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = get_session_path(&self.id);
        let data = serde_yaml::to_string(self).unwrap();
        fs::write(path, data)
    }

    pub fn load(id: &Uuid) -> Result<Self, std::io::Error> {
        let path = get_session_path(id);
        let data = fs::read_to_string(path)?;
        let session: Session = serde_yaml::from_str(&data).unwrap();
        Ok(session)
    }
}

fn get_session_path(id: &Uuid) -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("warpish_terminal");
    path.push("sessions");
    fs::create_dir_all(&path).unwrap();
    path.push(format!("{}.yml", id));
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation_and_save_load() {
        let session_name = "test_session";
        let session = Session::new(session_name);
        assert_eq!(session.name, session_name);

        let result = session.save();
        assert!(result.is_ok());

        let loaded_session = Session::load(&session.id).unwrap();
        assert_eq!(loaded_session.id, session.id);
        assert_eq!(loaded_session.name, session.name);

        // Clean up the test session file
        fs::remove_file(get_session_path(&session.id)).unwrap();
    }
}
