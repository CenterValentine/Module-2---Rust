# Overview

This rust application will incorporate some of the starting principles of LLM execution, data ingestion and normalization.  File ingestion and normalization are critical steps in sterilizing data to avoid critical errors in RAG operations. I will implement several core processes, namely detect, extract, annotate, and validate.  I will build on the file ingestion skills I learned in the prior C++ module.  Module requirements will be met as I implement library tools and logic to detect files, extract, annotate and validate them. The core logic will rule on what library tasks are needed for a given file to prepare it for chunking and vector embeddings (to be implemented beyond this module).  This application will be built with expandability in mind by utilizing adapters which allow expansion and portability of new or adapting needs to eventually scale to a fully functioning homemade LLM .


[Software Demo Video (loom <5)](https://www.loom.com/share/fef24901dfd5471c84ad09113cb56a40?sid=fce8bbd6-5a7a-4b99-87f0-5bc3de614231)

# Development Environment

Naturally, the rust documentation library was critical to efficiently handling different data types with ease.  You can the library dependencies used for this application below. You can also see the libraries.md to see the libraries and methodologies that I considered for this project. I studied the setups and typical features used for the applied libraries before writing code.  I spent a great deal of time studying basic rust building on my prior C++ knowledge and building on the similar infastructure but taking things much further than I did before.  I consulted AI tools to help develop a solid archecture that I create a file converter that was extremely resilient to naunced differences between all files!  AI was used to ask questions about the libraries in my given scenarios so I didn't have to spend as much time figuring out the best implimentation of a given library.  
[text](https://www.youtube.com/watch?v%3DTOhNW4U64Vw)
ammonia specializes in HTML sanitation as well as typical normilization. Used clean html whitespaces, etc.. See https://docs.rs/ammonia/latest/ammonia/
base64 was my way of getting something useful out of binary when I have nothing else to work with. Converts binary bits into ledible text. See https://docs.rs/base64/latest/base64/
html2text as the name implies converts html to to text. It is used to parse text inside of html. See https://docs.rs/html2text/latest/html2text/
quick-xml is an xml scraping tool. Used to parse text inside of xml. See https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Reader.html
leptess a rustium implimentation of tesseract powers the OCR technology.  See https://crates.io/crates/leptess?utm_source=chatgpt.com
image is a native image encoding/decoding tool that can perform basic manipulations to images.  Used to write ocr contents.  This was not fully implimented as intened due to time constraints but will easily be expanded later. See https://docs.rs/image/latest/image/
lingua tells you which language a given text is written it. This code is implimented in the ocr_auto_lang file but the file was not integrated due to time constraints. https://docs.rs/lingua/latest/lingua/
pdfium-render is by far the most difficult library I had to impliment because its core features are not available through within crate.  Pdfium performs critical pdf text conversion task.  I had to locally add this library to make my implimentation work without major changes.  Despite its lack of integration the library is well supported.  It is located at the root and a utility folder was created because of pdfiums implimentation across both pdf and image adapters. 

# Useful Websites

- [W3 School](https://w3schools.com/rust)
- [Rust docs](https://docs.rs) (see libraries, comments and above for urls to specific libraries)
- [OCR tutorial](https://transloadit.com/devtips/recognize-text-in-images-ocr-in-rust/)
- [Rust ioerror handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) and [this](https://doc.rust-lang.org/std/io/enum.ErrorKind.html)





# Future Work

- Integrate language detection for more accurate extractions.
- Improved binary capabilities.  Possibly working a binary implimentation into the adapters themselves.
- leptess/tesseract automatic quality specifications based on image metadata or mime type.
- Sanitize only feature
- Advanced pdf rendering for more advanced layouts (columns and images)
- Reading and writing utilities
- Advanced OCR capabilities.
- Fuzzy detection logic
- LLM vector embeddings



# File structure
src/
├── main.rs
├── lib.rs
├── adapters/
│   ├── mod.rs              // FileAdapter, FileRecord, Content, FileKind, adapter_for(..)
│   ├── text.rs             // TextFileAdapter
│   ├── binary.rs           // BinaryFileAdapter
│   ├── pdf.rs              // PdfAdapter (pdfium-render, pdftotext as a fallback)
│   ├── html.rs             // HtmlAdapter (ammonia/html2text)
│   ├── xml.rs              // XmlAdapter (quick-xml)
│   └── image.rs              // OcrAdapter (leptess/tesseract + optional pdf rasterization)
├── ingest/
│   └── mod.rs
└── normalize/
    └── mod.rs