// use std::f32::consts::E;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub mod text;
pub mod binary;

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
// submodules
mod text;
mod binary;
mod pdf;
mod html;
mod xml;
mod image;
// mod ocr;


// Adapter for a path
pub fn adapter_for(path: &Path) -> Box<dyn FileAdapter + Send + Sync> {
    match detect_file_type(path){
        "txt" | "md" | "csv" | "tsv" | "json" | "toml" | "yaml" | "yml" | "rs"  | "py" | "java" | "c" | "cpp" | "cs" => Box::new(text::TextFileAdapter),
        "html" | "htm" => Box::new(html::HtmlFileAdapter),
        "xml" | "opf" | "ncx" => Box::new(xml::XmlFileAdapter),
        "pdf" => Box::new(pdf::PdfFileAdapter),
        "png" | "jpg" | "jpeg" | "tif" | "tiff" | "bmp" | "gif" => Box::new(image::ImageFileAdapter),
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
