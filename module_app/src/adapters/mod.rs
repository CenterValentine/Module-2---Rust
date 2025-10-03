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

pub fn detect_file_type(path: &Path) -> FileType {
    const TRY_TEXT: &[&str] = & [
                "txt","md","csv","tsv","json","toml","yaml","yml","html","htm","xml","rs","py","java","c","cpp","cs",
    ];


    // |e| is a minature function
    // .and_then() is a method of Option<T>... Its like "chain if Some, else stop".  Lets you return None
    if let Some(ext) = path.extension().and_then(|e| e.to_str().map(|e| e.to_ascii_lowercase())){
        if TRY_TEXT.contains(&ext.as_str()){
            return FileType::Text;
        }
    }

    // Binary if file is NUL
    if let Ok(mut f) = File::open(path) {
        let mut buf = [0u8; 4096];
        if let Ok(n) = f.read(&mut buf) {
            if buf[..n].iter().any(|&b| b == 0){
                return FileType::Binary;
            }
        }

    }

    //default
FileType::Text

}


// Adapter for a path
pub fn adapter_for(path: &Path) -> Box<dyn FileAdapter + Send + Sync> {
    match detect_file_type(path){
        FileType::Text => Box::new(text::TextFileAdapter),
        FileType::Binary => Box::new(binary::BinaryFileAdapter),
    }
}

//
pub fn write_all(output_path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = output_path.parent() {
    fs::create_dir_all(parent)?;
    }
    let mut w_file = File::create(output_path)?;
    w_file.write_all(bytes)
}
