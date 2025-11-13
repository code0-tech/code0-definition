use std::path::PathBuf;
use std::io;
use serde_json;

#[derive(Debug)]
pub enum ReaderError {
    JsonError {
        path: PathBuf,
        error: serde_json::Error,
    },
    ReadFeatureError {
        path: String,
        source: Box<ReaderError>,
    },
    ReadDirectoryError {
        path: PathBuf,
        error: io::Error,
    },
    ReadFileError {
        path: PathBuf,
        error: io::Error,
    },
    DirectoryEntryError(io::Error),
}