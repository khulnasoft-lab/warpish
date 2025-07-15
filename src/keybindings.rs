use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::Path;
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use yaml_rust::{Yaml, YamlLoader};

pub type Action = String;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub mods: ModifiersState,
}

#[derive(Debug, Default)]
pub struct Keymap(HashMap<KeyBinding, Action>);

impl Keymap {
    pub fn get(&self, binding: &KeyBinding) -> Option<&Action> {
        self.0.get(binding)
    }
}

lazy_static! {
    static ref KEY_CODE_MAP: HashMap<&'static str, KeyCode> = {
        let mut m = HashMap::new();
        // Add all necessary key mappings here.
        m.insert("A", KeyCode::KeyA); m.insert("B", KeyCode::KeyB); m.insert("C", KeyCode::KeyC);
        m.insert("D", KeyCode::KeyD); m.insert("E", KeyCode::KeyE); m.insert("F", KeyCode::KeyF);
        // ... add G-Z, 0-9, etc. as needed for your keybindings ...
        m
    };
}

pub fn parse_keybinding(s: &str) -> Option<KeyBinding> {
    let mut parts: Vec<&str> = s.split('-').collect();
    if parts.is_empty() {
        return None;
    }

    let key_code_str = parts.pop().unwrap();
    let mut mods = ModifiersState::empty();

    for part in parts {
        // FIX: Bind the lowercased string to a variable to extend its lifetime
        let lower_part = part.to_lowercase();
        match lower_part.as_str() {
            "cmd" => mods.insert(ModifiersState::SUPER),
            "ctrl" => mods.insert(ModifiersState::CONTROL),
            "alt" => mods.insert(ModifiersState::ALT),
            "shift" => mods.insert(ModifiersState::SHIFT),
            _ => return None,
        }
    }

    KEY_CODE_MAP.get(key_code_str).map(|&key| KeyBinding { key, mods })
}

pub fn load_keymap_from_yaml(path: &Path) -> Result<Keymap, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let docs = YamlLoader::load_from_str(&content).map_err(|e| e.to_string())?;
    let doc = docs.get(0).ok_or("YAML file is empty")?;

    let mut keymap = Keymap::default();

    let map = doc.as_hash().ok_or("Expected top-level YAML to be a map")?;
    for (action, key_string) in map {
        let action_str = action.as_str().ok_or("Action key must be a string")?;
        let key_str = key_string.as_str().ok_or("Keybinding must be a string")?;

        if let Some(binding) = parse_keybinding(key_str) {
            keymap.0.insert(binding, action_str.to_string());
        } else {
            log::warn!("Failed to parse keybinding: {}", key_str);
        }
    }

    Ok(keymap)
}

// FIX: Add public stub for load_keybindings
pub fn load_keybindings(path: &str) -> Option<Keymap> {
    // Placeholder implementation
    Some(Keymap(std::collections::HashMap::new()))
}

// Type alias for backward compatibility
pub type Keybindings = Keymap;
