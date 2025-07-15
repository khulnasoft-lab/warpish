use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DriveError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parsing error for file '{0}': {1}")]
    YamlParsing(String, serde_yaml::Error),
    #[error("JSON parsing error for file '{0}': {1}")]
    JsonParsing(String, serde_json::Error),
    #[error("Home directory not found")]
    HomeDirNotFound,
}

// --- Data Models ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub default_value: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Workflow {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub description: String,
    #[serde(default)]
    pub arguments: Vec<Argument>,
    pub source_url: Option<String>,
    pub author_url: Option<String>,
    #[serde(default)]
    pub shells: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notebook {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct EnvVars {
    pub name: String,
    pub vars: HashMap<String, String>,
}

/// A polymorphic enum to represent any object that can be in the Drive.
#[derive(Debug, Clone)]
pub enum DriveObject {
    Workflow(Workflow, Metadata),
    Notebook(Notebook, Metadata),
    Prompt(Prompt, Metadata),
    EnvVars(EnvVars, Metadata),
}

// --- Management Logic ---

use crate::sum_tree::SumTree;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
    pub is_team: bool,
    pub objects: Vec<DriveObject>,
    pub object_weights: SumTree,
}

#[derive(Debug, Clone)]
pub struct DriveManager {
    pub personal_ws: Workspace,
    pub team_workspaces: Vec<Workspace>,
}

impl DriveManager {
    /// Initializes the Drive by scanning the user's home directory.
    pub fn new() -> Result<Self, DriveError> {
        let base_path = dirs::home_dir()
            .ok_or(DriveError::HomeDirNotFound)?
            .join(".warpish_drive");

        fs::create_dir_all(&base_path)?;

        // Load personal workspace
        let personal_path = base_path.join("personal");
        fs::create_dir_all(&personal_path)?;
        let (personal_objects, personal_weights) = load_objects_from_disk(&personal_path)?;
        let personal_ws = Workspace {
            name: "Personal".to_string(),
            path: personal_path,
            is_team: false,
            objects: personal_objects,
            object_weights: personal_weights,
        };
        
        // In a real app, we'd scan for all team dirs. Here we simulate one.
        let team_path = base_path.join("team_stark");
        fs::create_dir_all(&team_path)?;
        let (team_objects, team_weights) = load_objects_from_disk(&team_path)?;
        let team_ws = Workspace {
            name: "Team Stark".to_string(),
            path: team_path,
            is_team: true,
            objects: team_objects,
            object_weights: team_weights,
        };

        Ok(DriveManager {
            personal_ws,
            team_workspaces: vec![team_ws],
        })
    }
}

fn load_objects_from_disk(dir_path: &Path) -> Result<(Vec<DriveObject>, SumTree), DriveError> {
    let mut objects = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let file_name = path.display().to_string();
            // We want to process the content file, not the metadata file
            if file_name.ends_with(".meta.json") { continue; }

            let content = fs::read_to_string(&path)?;
            
            // Try to load metadata, or create default
            let meta_path = path.with_extension("meta.json");
            let metadata = if meta_path.exists() {
                let meta_content = fs::read_to_string(&meta_path)?;
                serde_json::from_str(&meta_content).map_err(|e| DriveError::JsonParsing(meta_path.display().to_string(), e))?
            } else {
                Metadata { id: Uuid::new_v4(), created_at: chrono::Utc::now(), updated_at: chrono::Utc::now(), author: None }
            };

            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let object = match ext {
                    "yaml" | "yml" => {
                        // For now, we'll assume YAML files are Workflows
                        let workflow = serde_yaml::from_str(&content).map_err(|e| DriveError::YamlParsing(file_name, e))?;
                        Some(DriveObject::Workflow(workflow, metadata))
                    },
                    "md" => {
                        let notebook = Notebook { name: path.file_stem().unwrap().to_string_lossy().to_string(), content };
                        Some(DriveObject::Notebook(notebook, metadata))
                    }
                    _ => None
                };
                if let Some(obj) = object {
                    objects.push(obj);
                }
            }
        }
    }
    let mut sum_tree = SumTree::new(objects.len());
    for (i, _) in objects.iter().enumerate() {
        sum_tree.set(i, 1.0);
    }
    Ok((objects, sum_tree))
} 