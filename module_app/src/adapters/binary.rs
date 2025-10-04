use super::{write_all, Content, FileAdapter, FileType, FileRecord};
use std::fs;
use std::io;
use std::path::Path;
// converts bytes to characters
use base64::{engine::general_purpose, Engine as _};

pub struct BinaryFileAdapter;

impl FileAdapter for BinaryFileAdapter {
    fn read(&self, path: &Path) -> io::Result<FileRecord> {
        let bytes = fs::read(path)?;
        Ok(FileRecord {
            path:path.to_path_buf(),
            kind: FileType::Binary,
            content: Content::Bytes(bytes),
        })
    }

    fn write(&self, record: &FileRecord, output_path: &Path) -> io::Result<()> {
        match &record.content {

            Content::Bytes(b) => {
                let encoded = general_purpose::STANDARD.encode(b);
            write_all(output_path, encoded.as_bytes())
        }
            Content::Text(s) => write_all(output_path, s.as_bytes()),
        }
    }
}