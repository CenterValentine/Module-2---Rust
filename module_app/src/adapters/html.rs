use std::{fs, io, path::Path};
use crate::adapters::{Content, FileRecord, FileType, FileAdapter};

pub struct HtmlFileAdapter;

impl FileAdapter for HtmlFileAdapter {
    fn read(&self, path: &Path) -> io::Result<FileRecord> {
        let html = fs::read_to_string(path)?;
        // use ammonia to clean html.
        let sanitized = ammonia::clean(&html);
        // use html2text to convert html2text.  We may want the html tags and can include this once we know what is best for the LLM.
        let text = html2text::from_read(sanitized.as_bytes(), 999_999);


        Ok(FileRecord {
            path: path.to_path_buf(),
            kind: FileType::Html,
            content: Content::Text(text),
        })
    }

    fn write(&self, record: &FileRecord, output_path: &Path) -> io::Result<()> {
        match &record.content {
            Content::Text(s) => std::fs::write(output_path, s),
            Content::Bytes(b) => std::fs::write(output_path, b), 
        }
    }
}