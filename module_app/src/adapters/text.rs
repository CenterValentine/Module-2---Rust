use super::{write_all, Content, FileAdapter, FileType, FileRecord};
use std::fs;
use std::io;
use std::path::Path;

pub struct TextFileAdapter;


impl FileAdapter for TextFileAdapter {
    fn read(&self, path: &Path) -> io::Result<FileRecord> {
        let text = fs::read_to_string(path)?;
        Ok(FileRecord {
            path:path.to_path_buf(),
            kind: FileType::Text,
            content: Content::Text(text),
        })
    }

    fn write(&self, record: &FileRecord, output_path: &Path) -> io::Result<()> {
        match &record.content {
Content::Text(s) => write_all(output_path, s.as_bytes()),
Content::Bytes(_) => Err(io::Error::new(
    io::ErrorKind::InvalidInput,
    "Text adapter cannot write byte data",
))
        }
    }
}