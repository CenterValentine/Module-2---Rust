use std::io;
use pdfium_render::prelude::*;

pub fn new_pdfium() -> io::Result<Pdfium> {
    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("PDFium bind: {e}")))?;
    Ok(Pdfium::new(bindings))
}