// use std::f32::consts::E;
use std::fs::{self, File};
use std::io::{Write};
use std::path::{Path, PathBuf};

use crate::adapters::pdf::PdfAdapter;
pub use binary::BinaryFileAdapter;

// submodules
pub mod text;
pub mod binary;
pub mod pdf;
pub mod html;
pub mod xml;
pub mod image;
pub mod ocr_auto_lang;
pub mod util;
// mod ocr;

//Acceptable file types
pub enum FileType{
    Text,
    Binary,
    Html,
    Xml,
    Pdf,
    Image, 
}

// The represenation of the file and it's contents.
pub struct FileRecord{
    pub path: PathBuf,
    pub kind: FileType,
    pub content: Content,
}

// The data contents of the file.
pub enum Content {
    Text(String),
    Bytes(Vec<u8>),
}

//Actual adapter interface "class"
pub trait FileAdapter {
    fn read(&self, path: &Path) -> std::io::Result<FileRecord>;
    fn write(&self, record: &FileRecord, output_path: &Path) -> std::io::Result<()>;
}



// Adapter for a path
pub fn adapter_for(path: &Path) -> Box<dyn FileAdapter + Send + Sync> {
    let ext = path
    .extension()
    .and_then(|s| s.to_str())
    .map(|s| s.to_ascii_lowercase())
    .unwrap_or_default();

    match ext.as_str(){
        "txt" | "md" | "csv" | "tsv" | "json" | "toml" | "yaml" | "yml" | "rs"  | "py" | "java" | "c" | "cpp" | "cs" => Box::new(text::TextFileAdapter),
        "html" | "htm" => Box::new(html::HtmlFileAdapter),
        "xml" | "opf" | "ncx" => Box::new(xml::XmlAdapter),
        "pdf" => Box::new(PdfAdapter),
        "png" | "jpg" | "jpeg" | "tif" | "tiff" | "bmp" | "gif" => Box::new(image::OcrImageAdapter::images()),
        _ => Box::new(BinaryFileAdapter),
    }
}

//writes any number of files
pub fn write_all(output_path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = output_path.parent() {
    fs::create_dir_all(parent)?;
    }
    let mut w_file = File::create(output_path)?;
    w_file.write_all(bytes)
}


