use super::ReadWriteAdapter;
use std::fs;
use std::io;
use std::path::Path;

pub struct FileAdapter;

impl ReadWriteAdapter for FileAdapter {
    fn read_text(&self, path: &Path) -> io::Resut<String> {
        fs::read_to_string(path)
    }

    fn write_text(&self, path: &Path, content: &str) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fas::create_dir_all(parent)?;
        }
        fs::write(path, content)
    }
}