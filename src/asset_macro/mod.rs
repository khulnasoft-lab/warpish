//! Asset Macro
//! 
//! This module provides compile-time asset embedding functionality,
//! allowing static assets to be embedded directly into the binary.

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

/// Macro for embedding assets at compile time
#[macro_export]
macro_rules! embed_asset {
    ($path:expr) => {
        Assets::get($path).expect(&format!("Asset not found: {}", $path))
    };
}

/// Get an embedded asset by path
pub fn get_asset(path: &str) -> Option<rust_embed::EmbeddedFile> {
    Assets::get(path)
}

/// List all embedded assets
pub fn list_assets() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
    Assets::iter()
}

/// Check if an asset exists
pub fn asset_exists(path: &str) -> bool {
    Assets::get(path).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_exists() {
        // Test would verify that known assets exist
        // assert!(asset_exists("some_known_asset.txt"));
    }
}
