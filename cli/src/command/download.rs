use bytes::Bytes;
use colored::*;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use zip::ZipArchive;
use crate::formatter::{error_without_trace, info, success};

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
        Some(bytes) => {
            success(format!("Successfully downloaded {} bytes", bytes.len()));
            bytes
        }
        None => {
            error_without_trace(String::from("Download failed."));
            return;
        }
    };

    // Extract the zip file
    info("Extracting definitions...".to_string());
    convert_bytes_to_folder(bytes, zip_path).await;

    // Handle feature filtering if specified
    if let Some(selected_features) = features {
        info(format!("Extracted features: {:?} ", selected_features));
        filter_features(selected_features).await;
    } else {
        info("Extracted all features!".to_string());
    }

    success(format!("Download was successful. Definitions are now available in {out_folder_path}."));
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
        Some(asset) => {
            asset
        }
        None => {
            panic!("Definition folder is not called `definitions.zip` and was not inside the asset folder of the GitHub release!");
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
                        println!(
                            "{} {}",
                            "✅".green(),
                            "Download completed successfully".green()
                        );
                        Some(bytes)
                    }
                    Err(e) => {
                        println!(
                            "{} {}",
                            "❌".red(),
                            format!("Failed to read download data: {e}").red()
                        );
                        None
                    }
                }
            } else {
                println!(
                    "{} {}",
                    "❌".red(),
                    format!("Download failed with status: {}", response.status()).red()
                );
                None
            }
        }
        Err(e) => {
            println!(
                "{} {}",
                "❌".red(),
                format!("Download request failed: {e}").red()
            );
            None
        }
    }
}

async fn convert_bytes_to_folder(bytes: Bytes, zip_path: &str) {
    println!("{} Writing zip file to disk...", "💾".bright_blue());

    if let Err(e) = fs::write(zip_path, &bytes) {
        println!(
            "{} {}",
            "❌".red(),
            format!("Failed to write zip file: {e}").red()
        );
        return;
    }
    println!(
        "{} {}",
        "✅".green(),
        "Zip file written successfully".green()
    );

    println!("{} Opening zip archive...", "📂".bright_blue());
    let zip_file = match File::open(zip_path) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "{} {}",
                "❌".red(),
                format!("Failed to open zip file: {e}").red()
            );
            return;
        }
    };

    let mut archive = match ZipArchive::new(zip_file) {
        Ok(archive) => {
            println!(
                "{} {}",
                "✅".green(),
                format!("Successfully opened archive with {} files", archive.len()).green()
            );
            archive
        }
        Err(e) => {
            println!(
                "{} {}",
                "❌".red(),
                format!("Failed to read zip archive: {e}").red()
            );
            return;
        }
    };

    println!("{} Extracting files...", "📤".bright_blue());
    let mut extracted_count = 0;
    let total_files = archive.len();

    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(e) => {
                println!(
                    "{} {}",
                    "⚠️".yellow(),
                    format!("Warning: Failed to read file at index {i}: {e}").yellow()
                );
                continue;
            }
        };

        let out_path = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        if file.name().ends_with('/') {
            if let Err(e) = std::fs::create_dir_all(&out_path) {
                println!(
                    "{} {}",
                    "⚠️".yellow(),
                    format!(
                        "Warning: Failed to create directory {}: {}",
                        out_path.display(),
                        e
                    )
                    .yellow()
                );
            }
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    if let Err(e) = std::fs::create_dir_all(p) {
                        println!(
                            "{} {}",
                            "⚠️".yellow(),
                            format!(
                                "Warning: Failed to create parent directory {}: {}",
                                p.display(),
                                e
                            )
                            .yellow()
                        );
                        continue;
                    }
                }
            }

            match File::create(&out_path) {
                Ok(mut outfile) => {
                    if let Err(e) = std::io::copy(&mut file, &mut outfile) {
                        println!(
                            "{} {}",
                            "⚠️".yellow(),
                            format!("Warning: Failed to extract {}: {}", out_path.display(), e)
                                .yellow()
                        );
                    } else {
                        extracted_count += 1;
                    }
                }
                Err(e) => {
                    println!(
                        "{} {}",
                        "⚠️".yellow(),
                        format!(
                            "Warning: Failed to create file {}: {}",
                            out_path.display(),
                            e
                        )
                        .yellow()
                    );
                }
            }
        }
    }

    println!(
        "{} {}",
        "✅".green(),
        format!("Successfully extracted {extracted_count}/{total_files} files").green()
    );

    // Clean up zip file
    println!("{} Cleaning up temporary files...", "🧹".bright_blue());
    match fs::remove_file(zip_path) {
        Ok(_) => println!("{} {}", "✅".green(), "Temporary zip file removed".green()),
        Err(e) => println!(
            "{} {}",
            "⚠️".yellow(),
            format!("Warning: Failed to remove temporary zip file: {e}").yellow()
        ),
    }
}

async fn filter_features(selected_features: Vec<String>) {
    let definitions_path = "./definitions";

    match fs::read_dir(definitions_path) {
        Ok(entries) => {
            let mut removed_count = 0;
            let mut kept_count = 0;

            for entry in entries {
                let directory = match entry {
                    Ok(directory) => directory,
                    Err(e) => {
                        println!(
                            "{} {}",
                            "⚠️".yellow(),
                            format!("Warning: Failed to read directory entry: {e}").yellow()
                        );
                        continue;
                    }
                };

                let name = directory.file_name().to_str().unwrap_or("").to_string();

                if !selected_features.contains(&name) {
                    println!("  {} Removing feature: {}", "🗑️".red(), name.red());
                    match fs::remove_dir_all(directory.path()) {
                        Ok(_) => {
                            println!("    {} Successfully removed", "✅".green());
                            removed_count += 1;
                        }
                        Err(e) => {
                            println!(
                                "    {} Failed to remove: {}",
                                "❌".red(),
                                e.to_string().red()
                            );
                        }
                    }
                } else {
                    println!("  {} Keeping feature: {}", "📁".green(), name.green());
                    kept_count += 1;
                }
            }

            println!("\n{} Feature filtering completed:", "📊".bright_blue());
            println!(
                "  {} Features kept: {}",
                "✅".green(),
                kept_count.to_string().green().bold()
            );
            println!(
                "  {} Features removed: {}",
                "🗑️".red(),
                removed_count.to_string().red().bold()
            );
        }
        Err(e) => {
            println!(
                "{} {}",
                "❌".red(),
                format!("Failed to read definitions directory: {e}").red()
            );
        }
    }
}
