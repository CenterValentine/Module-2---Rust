use std::{io, path::Path};
use crate::adapters::{Content, FileAdapter, FileRecord, FileType};


pub struct PdfAdapter;

impl FileAdapter for PdfAdapter{
    fn read (&self, path: &Path) -> io::Result<()> {

        // use pdfium 
        match extract_with_pdfium(path) {
            Ok(text) if !text.trim().is_empty() => {
                return Ok(FileRecord {
                    path: path.to_path_buf(),
                    kind: FileType::Pdf,
                    content: Content::Text(text),
                });
            }
        _=> {}
        }

        // pdftotext as fallback
        if let Ok(text) = extract_with_pdftotext (path) {
            return Ok(FileRecord {
                path: path.to_path_buf(),
                kind: FileType::Pdf,
                content: Content::Text(text),
            });
        }


        //  raw bytes pdf for all other cases.
        let bytes = std::fs::read(path)?;
        Ok(FileRecord {
            path: path.to_path_buf(),
            kind: FileType::Pdf,
            content: Content::Text(bytes)
        })
    }


    fn write (&self, record: &FileRecord, output_path: &Path) -> io::Result<()> {
        match &record.content {
            Content::Text(s) => std::fs::write(output_path, s),
            Content::Bytes(b) => std::fs::write(output_path, b),
        }

    }


}

fn extract_with_pdfium(path: &Path) -> io::Result<()> {
    // This is how pdfium is initiated. (https://docs.rs/pdfium-render/latest/pdfium_render/)
    use pdfium_render::prelude::*;
// bind
    let pdfium = Pdfium::new(Pdfium::bind_to_system_library()
    //catch pdfium errors
    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("PDFium bind: {e}")))?);
// passwords can be added to 2nd argument of load_pdf_from_file
    let doc = pdfium.load_pdf_from_file(path, None)
    // catch load_pdf_from_file errors
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("PDF open: {e}")))?;
// output declaration
    let mut out = String::new();

    // Index range allows for potential page control or multiple passes.
    for page_index in 0..doc.pages().len() {
        // use rayon can be used to enable simultaneous processing of pages. Good for OCR.
        let page = doc.pages().get(page_index).unwrap();
        // entire page
        let page_text = page.text().unwrap().all();
// potential feature expansion for columned pdf and other pdf formats.
        out.push_str(&page_text);
// line break
        out.push('\n');
    }

// alternative loop for safer iterations, with borrowing. 
//  for (index, page) in document.pages().iter().enumerate() {
// if let Ok(ptext) = page.text() {
//              out.push_str(&ptext.all());
//              out.push('\n');
// } else {
// eprintln!("Failed to extract text from page {}", index + 1);
// }
    Ok(out)

}

fn extract_with_pdftotext(path: &Path) -> io::Result<String>{
    // Access pdftotext https://docs.rs/pdftotext using CLI commands.
    use std::process::Command;
    let output = Command::new("pdftotext")
    .arg("-layout")
    .arg(path)
    .arg("-")
    .output();

    match output{
        // lossy doesn't fail, just whatever text it can extract.
        Ok(out) if out.status.success() => Ok(String::from_utf8_lossy(&out.stdout)
        // forces a string regardless of ownership.
            .into_owned()),
        Ok(out) => Err(io::Error::new(io::ErrorKind::Other, format!("pdftotext failed: {:?}", out.status))),
        Err(e) => Err(io::Error::new(io::ErrorKind::NotFound, e)),
    }


}



