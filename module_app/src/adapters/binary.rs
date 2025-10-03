use super::{write_all, Content, FileAdapter, FileType, FileRecord};
use std::fs;
use std::io;
use std::path::Path;

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
            Content::Bytes(b) => write_all(output_path, b),
Content::Text(s) => write_all(output_path, s.as_bytes()),
        }
    }
}