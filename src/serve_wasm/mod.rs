//! WASM Serving Module
//!
//! This module provides a simple web server to serve WASM files and other static assets.

use warp::Filter;

/// Starts the WASM server.
///
/// # Arguments
///
/// * `port` - The port to listen on.
/// * `path` - The path to the directory to serve.
///
/// # Panics
///
/// This function will panic if the server fails to start.
#[tokio::main]
pub async fn start_server(port: u16, path: &str) {
    let wasm_path = warp::path("wasm").and(warp::fs::dir(path.to_string()));

    println!("[WASM Server] Starting on port {}", port);
    println!("[WASM Server] Serving files from: {}", path);

    warp::serve(wasm_path).run(([127, 0, 0, 1], port)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use reqwest::Client;

    #[tokio::test]
    async fn test_start_server() {
        let port = 3031; // Use a different port for testing
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap().to_string();

        // Create a dummy wasm file
        let file_path = dir.path().join("test.wasm");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"test wasm content").unwrap();

        // Spawn the server in a separate task
        tokio::spawn(async move {
            start_server(port, &path).await;
        });

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Make a request to the server
        let client = Client::new();
        let res = client
            .get(&format!("http://localhost:{}/wasm/test.wasm", port))
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        assert_eq!(res.text().await.unwrap(), "test wasm content");
    }
}
