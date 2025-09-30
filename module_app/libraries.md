
## Parsing:

### text/MD
-  [comrak]()
    - format_xml
    - markdown_to_html
    - parse_document
-  [regex]()
-  [unicode-segmentation](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/)
    - Graphemes|+Indices |+Cursor
    - UWordBounds|+Indices
    - USentenceBounds|+Indices
    -Unicode*^ (any of the above)

### [html]()
- scraper
    - `Html::parse_fragment(text);`
    - Selector::parse("h1.foo").unwrap();

```rust 
for element in fragment.select(&selector){  assert_eq!("li", element.value().name());
}
```

### [pdf]()
- pdfium-render
    - pdfium_render::prelude::*;
        - let pdfium = Pdfium::default();
        - let doc = pdfium.load_pdf_from_file(path, /* password */ None)?;
        - iterate through pages looping `doc.pages().iter().enumerate()`
            - `page.text().all();`
        -4. “Recovering order if needed”
> draw order ≠ reading order
    - Text in PDFs might come out scrambled , you can rebuild human-readable flow by:

	1.	Collecting all segments/rectangls (rects) from a page.
	2.	Sort them by their vertical position (top → bottom), then horizontal (left → right).
	•	For multi-column: detect wide gaps. Group text into “columns,”. Merge columns top-to-bottom.
	3.	Join them with spacing rules:
	•	If the horizontal gap is small → same line.
	•	If vertical gap is bigger → new paragraph/line break.
	4.	Optionally: detect rotated text (rectangles that aren’t horizontal) and decide whether to keep/ignore (e.g., margins, headers).

- lopdf
    - decode_text_string
    - encode_utf8 (no recommended)
    - encode_utf16_be
    - substr/substring/text_string
- pdftotext
    - pdftotext::pdftotext_layout
        - pub fn pdftotext_layout(filename: &str) -> Result<Vec<String>, Error>

### [epub]()
- epub create ([unzip](https://docs.rs/unzip/latest/unzip/)/xml)
    - epub::doc::EpubDoc
        - let doc = EpubDoc::new("test.epub");
        - assert!(doc.is_ok());
        - let doc = doc.unwrap();
    - get metadata from HashMap
        - let title = doc.mdata("title");
        assert_eq!(title.unwrap(), "Todo es mio");
- quick-xml
    - use quick_xml::events::Event; 
    - use quick_xml::reader::Reader;
    - let mut reader = Reader::from_str(xml);
    - reader.config_mut().trim_text(true);

    - escape-html
    - overlapped-lists: Infers from tag declarations an Object Oriented struct of tags.
    - html_escape


### [docx]()
- zip
- quick-xml

### [OCR]()
- tesseract-rs
    - Coming back to this when I have more context. 
- leptess

### [Language detect]()
- whatlang
- lingua

### [de-boilerplate]()
- readability
- html2text
