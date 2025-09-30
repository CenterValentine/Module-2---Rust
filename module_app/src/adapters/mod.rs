pub mod file;

use std::io;
use std::path::Path;

pub trait ReadWriteAdapter {
    fn read_text(&self, path: &Path) -> io::Result<String>;
    fn write_text(&self, path: &Path, content: &str) -> io::Result<()>;
}