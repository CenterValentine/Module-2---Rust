
use std::{io, path::Path};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use crate::adapters::{FileAdapter, FileRecord};
use crate::adapters::image::OcrImageAdapter;


fn lang_detector() -> LanguageDetector {
    use Language::*;
    let langs = vec![
        Arabic, Hebrew, Japanese, Korean, Thai,
        Chinese, Russian, Greek, Spanish, French, German, English,
    ];
    LanguageDetectorBuilder::from_languages(&langs).build()
}

/// Map Lingua's language to a Tesseract traineddata code.
fn lingua_to_tesseract(lang: Language) -> &'static str {
    use Language::*;
    match lang {
        Arabic => "ara",
        Hebrew => "heb",
        Japanese => "jpn",
        Korean => "kor",
        Thai => "tha",
        Chinese => "chi_sim",   // or "chi_tra" depending on your data
        Russian => "rus",
        Greek => "ell",
        Spanish => "spa",
        French => "fra",
        German => "deu",
        English => "eng",
        _ => "eng",
    }
}


fn guess_lang(detector: &LanguageDetector, text: &str) -> Option<&'static str> {
    detector.detect_language_of(text).map(lingua_to_tesseract)
}


pub fn ocr_image_auto_lang(p: &Path) -> io::Result<FileRecord> {
    // First pass: broad OCR to get a snippet (combine likely languages)
    let broad = "eng+ara+heb+jpn+kor+tha+chi_sim+rus+ell+spa+fra+deu";
    let tmp = OcrImageAdapter::images().with_lang(broad).read(p)?;

    let snippet = match &tmp.content {
    crate::adapters::Content::Text(s) => s,
        _ => "",
    };

    // Detect language
    let detector = lang_detector();
    if let Some(code) = guess_lang(&detector, snippet) {
        // Second pass: do a clean OCR with the detected language only
        let final_rec = OcrImageAdapter::images().with_lang(code).read(p)?;
        return Ok(final_rec);
    }

    // Fallback broad result
    Ok(tmp)
}


pub fn ocr_pdf_auto_lang(p: &Path) -> io::Result<FileRecord> {
    let broad = "eng+ara+heb+jpn+kor+tha+chi_sim+rus+ell+spa+fra+deu";
    let tmp = OcrImageAdapter::pdf_pages().with_lang(broad).read(p)?;

    let snippet = match &tmp.content {
        crate::adapters::Content::Text(s) => s,
        _ => "",
    };

    let detector = lang_detector();
    if let Some(code) = guess_lang(&detector, snippet) {
        let final_rec = OcrImageAdapter::pdf_pages().with_lang(code).read(p)?;
        return Ok(final_rec);
    }

    Ok(tmp)
}