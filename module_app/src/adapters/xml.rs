use std::{fs::File, io::{self, BufReader}, path::Path};
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::adapters::{Content, FileRecord, FileAdapter, FileType};

pub struct XmlAdapter;

impl FileAdapter for XmlAdapter {
    fn read(&self, path:&Path) -> io::Result<FileRecord> {


        let file = File::open(path)?;
        let mut reader = Reader::from_reader(BufReader::new(file));
reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut out = String::new();

        // Get XML events until Eof.
        loop {
            match reader.read_event_into(&mut buf){
                // text content inside the quick_xml Event.
                Ok(Event::Text(t)) => {


                out.push_str(&t
                    // xml escapes such as &amp; -> &
                .unescape()
                // returns empty string if unescape fails
                .unwrap_or_default());
            }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e));
                },
                _ => {}
            }
            buf.clear();
        }
        Ok(FileRecord {
            path: path.to_path_buf(),
            kind: FileType::Xml,
            content: Content::Text(out),
        })
    }

    fn write (&self, record: &FileRecord, output_path: &Path) -> io::Result<()> {
        match &record.content {
            Content::Text(s) => std::fs::write(output_path, s),
            Content::Bytes(b) => std::fs::write(output_path, b),
        }
    }

}