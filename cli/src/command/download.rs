use crate::formatter::{error_without_trace, info, success};
use bytes::Bytes;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
    size: u64,
}

pub async fn handle_download(tag: Option<String>, features: Option<Vec<String>>) {
    let out_folder_path = "./definitions";
    let zip_path = "./definitions.zip";

    // Download the definitions
    info("Starting download process...".to_string());
    let bytes = match download_definitions_as_bytes(tag).await {
        Some(bytes) => bytes,
        None => {
            error_without_trace(String::from("Download failed."));
            return;
        }
    };

    // Extract the zip file
    convert_bytes_to_folder(bytes, zip_path).await;

    // Handle feature filtering if specified
    if let Some(selected_features) = features {
        info(format!("Extracted features: {:?} ", selected_features));
        filter_features(selected_features).await;
    } else {
        info("Extracted all features!".to_string());
    }

    let path = Path::new(out_folder_path);
    success(format!(
        "Download was successful. Definitions are now available: {}.",
        path.display()
    ));
}

async fn download_definitions_as_bytes(tag: Option<String>) -> Option<bytes::Bytes> {
    let client = reqwest::Client::new();

    let url = match tag {
        Some(t) => {
            info(format!("Selected the version: {}", t));
            format!("https://api.github.com/repos/code0-tech/code0-definition/releases/tags/{t}")
        }
        None => {
            info("No version specified, using latest version".to_string());
            String::from("https://api.github.com/repos/code0-tech/code0-definition/releases/latest")
        }
    };

    let release_request = match client
        .get(url)
        .header(USER_AGENT, "code0-definition-cli")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                response
            } else {
                return None;
            }
        }
        Err(e) => {
            panic!("Request failed: {}", e);
        }
    };

    let release: Release = match release_request.json::<Release>().await {
        Ok(release) => {
            info(format!("Selected release: {}", release.tag_name));
            release
        }
        Err(e) => {
            panic!("Request failed: {}", e);
        }
    };

    let asset = match release
        .assets
        .into_iter()
        .find(|a| a.name == "definitions.zip")
    {
        Some(asset) => asset,
        None => {
            panic!(
                "Definition folder is not called `definitions.zip` and was not inside the asset folder of the GitHub release!"
            );
        }
    };

    match client
        .get(&asset.browser_download_url)
        .header(USER_AGENT, "code0-definition-cli")
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => {
                        info("Download completed successfully".to_string());
                        Some(bytes)
                    }
                    Err(e) => {
                        error_without_trace(format!("Failed to read download data: {e}"));
                        None
                    }
                }
            } else {
                error_without_trace(format!(
                    "Download failed with status: {}",
                    response.status()
                ));
                None
            }
        }
        Err(e) => {
            panic!("Download request failed: {e}");
        }
    }
}

async fn convert_bytes_to_folder(bytes: Bytes, zip_path: &str) {
    if let Err(e) = fs::write(zip_path, &bytes) {
        panic!("Failed to write zip file: {e}")
    }

    let zip_file = match File::open(zip_path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to open zip file: {e}");
        }
    };

    let mut archive = match ZipArchive::new(zip_file) {
        Ok(archive) => archive,
        Err(e) => {
            panic!("Failed to read zip archive: {e}");
        }
    };

    info("Extracting files...".to_string());
    let total_files = archive.len();

    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(e) => {
                panic!("Failed to read file at index {i}: {e}");
            }
        };

        let out_path = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        if file.name().ends_with('/') {
            if let Err(e) = fs::create_dir_all(&out_path) {
                panic!("Failed to create directory {}: {}", out_path.display(), e);
            }
        } else {
            if let Some(p) = out_path.parent()
                && !p.exists()
                    && let Err(e) = fs::create_dir_all(p) {
                        panic!(
                            "Warning: Failed to create parent directory {}: {}",
                            p.display(),
                            e
                        );
                    }

            match File::create(&out_path) {
                Ok(mut outfile) => {
                    if let Err(e) = std::io::copy(&mut file, &mut outfile) {
                        panic!("Warning: Failed to extract {}: {}", out_path.display(), e);
                    }
                }
                Err(e) => {
                    panic!("Failed to create file {}: {}", out_path.display(), e);
                }
            }
        }
    }

    info(format!("Successfully extracted {total_files} files"));
    info("Cleaning up temporary files...".to_string());

    match fs::remove_file(zip_path) {
        Ok(_) => info("Temporary zip file removed".to_string()),
        Err(e) => error_without_trace(format!("Warning: Failed to remove temporary zip file: {e}")),
    }
}

async fn filter_features(selected_features: Vec<String>) {
    let definitions_path = "./definitions";

    match fs::read_dir(definitions_path) {
        Ok(entries) => {
            for entry in entries {
                let directory = match entry {
                    Ok(directory) => directory,
                    Err(e) => {
                        panic!(
                            "{}",
                            format!("Warning: Failed to read directory entry: {e}")
                        );
                    }
                };

                let name = directory.file_name().to_str().unwrap_or("").to_string();

                if !selected_features.contains(&name) {
                    match fs::remove_dir_all(directory.path()) {
                        Ok(_) => {}
                        Err(e) => {
                            error_without_trace(format!("Warning: Failed to remove directory: {e}"))
                        }
                    }
                }
            }
        }
        Err(e) => {
            error_without_trace(format!("Failed to read definitions directory: {e}"));
        }
    }
}
