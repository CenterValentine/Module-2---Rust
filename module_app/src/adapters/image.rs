use std::{io, path::Path};
use crate::adapters::{Content, FileAdapter, FileType, FileRecord};


// Leptess - Tesseract binder - https://crates.io/crates/leptess?utm_source=chatgpt.com
pub struct OcrAdapter {
mode: OcrMode,
lang: String, //default to "eng"
}

enum OcrMode {
ImagesOnly,
Pdfium
}

// Adapter implimentations
impl OcrAdapter {
    pub fn images() -> Self {
        Self {mode: OcrMode::ImagesOnly, lang: "eng".into() }
    }
    pub fn pdf_pages() -> Self {
        Self {mode: OcrMode::Pdfium, lang: "eng".into()}
    }
    pub fn with_lang(mut self, lang: &str) -> Self {
        self.lang = lang.into();
        self
    }

}


impl FileAdapter for OcrAdapter {
    fn read(&self, path:&Path) -> io::Result<FileRecord> {
        let text = match self.mode {
            OcrMode::ImagesOnly => ocr_standalone_image(path, &self.lang)?,
            OcrMode::Pdfium => ocr_pdf_by_rasterizing(path, &self.lang)?,
        };

        Ok(FileRecord {
            path: path.to_path_buf(),
            kind: FileType::Image,
            content: Content::Text(text),
        })

    }

    fn write(&self, record: &FileRecord, output_path: &Path) -> io::Result<()> {
        match &record.content {
            Content::Text(s) => std::fs::write(output_path, s),
            Content::Bytes(b) => std::fs::write(output_path,b),
        }
    }
}


// OCR libraries

fn ocr_standalone_image(path: &Path, lang: &str) -> io::Result<String> {
    use leptess::{LepTess, Variable};
    let mut lt = LepTess::new(None,lang)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("tesseract init: {e}")))?;
    lt.set_variable(Variable::TesseditCharWhitelist, "") // optional adjustments
    .ok();
    lt.set_image(path.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "bad utf path"))?);
    lt.get_utf8_text()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("ocr error: {e}")))

}

fn ocr_pdf_by_rasterizing(){
    use pdfium_render::prelude::*;
    use leptess::LepTess;

    let pdfium = Pdfium::new(Pdfium::bind_to_system_library()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Pdfium binding error: {e}")))?;

    let document = pdfium.load_pdf_from_file(path,None)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Pdf open error: {e}")))?;

    let mut out = String::new();
    // indexed pages allows for more versitile navigation of  pages.  See pdf adapter for more information.
    for idx in 0..doc.pages().len() {
        let page = document.pages().get(idx).unwrap();
        let bmp = page.render_with_config(
            &PdfRenderConfig::new()
                .set_target_width(2000)             // Expand adjust quality preference
                .render_form_data(true)).unwrap().as_image(); //image::DynamicImage

        // Save to temporary png, stream to tesseract
        let mut lt = LepTess::new(None, lang)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("tesseract init: {e}")))?;
        let png = {
            let mut buf = Vec::new();
            bmp.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageOutputFormat::Png)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("png encode error: {e}")))?;
            buf
        };
        lt.set_image_from_mem(&png_bytes).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("set_image error: {e}")))?;
        let page_test = lt.get_utf8_text().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("ocr error: {e}")))?;
        out.push_str(&page_text);
        out.push('\n');
    }
    Ok(out)

}