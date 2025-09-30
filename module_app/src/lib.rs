pub mod adapters;
pub mod ingest;
pub mod normalize;

use std::collections::HashMap;
use std::path::PathBuf;

pub struct RawDocument {
    pub path: PathBuf,
    pub mime: String,
    pub metadata: HashMap<String, String>,
    pub content: String,
}

pub struct NormalizedDocument {
    pub path: PathBuf,
    pub mime: String,
    pub metadata: HashMap<String, String>,
    pub text: String,
}