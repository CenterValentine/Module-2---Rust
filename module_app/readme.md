# Overview

This rust application will incorporate some of the starting principles of LLM execution, data ingestion and normalization.  File ingestion and normalization are critical steps in sterilizing data to avoid critical errors in RAG operations. I will implement several core processes, namely detect, extract, annotate, and validate.  I will build on the file ingestion skills I learned in the prior C++ module.  Module requirements will be met as I implement library tools and logic to detect files, extract, annotate and validate them. The core logic will rule on what library tasks are needed for a given file to prepare it for chunking and vector embeddings (to be implemented beyond this module).  This application will be built with expandability in mind by utilizing adapters which allow expansion and portability of new or adapting needs to eventually scale to a fully functioning homemade LLM .

{Provide a description of the software that you wrote to demonstrate the Rust language.}

{Describe your purpose for writing this software.}

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

{Describe the tools that you used to develop the software}

{Describe the programming language that you used and any libraries.}

# Useful Websites

- [W3 School](http://url.link.goes.here)
- [Web Site Name](http://url.link.goes.here)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}

- Item 1
- Item 2
- Item 3



# File structure
src/
├── main.rs
├── lib.rs
├── adapters/
│   ├── mod.rs              // FileAdapter, FileRecord, Content, FileKind, adapter_for(..)
│   ├── text.rs             // TextFileAdapter
│   ├── binary.rs           // BinaryFileAdapter
│   ├── pdf.rs              // PdfAdapter (pdfium-render, pdftotext as a fallback)
│   ├── html.rs             // HtmlAdapter (scraper + optional ammonia/html2text)
│   ├── xml.rs              // XmlAdapter (quick-xml)
│   └── ocr.rs              // OcrAdapter (leptess/tesseract + optional pdf rasterization)
├── ingest/
│   └── mod.rs
└── normalize/
    └── mod.rs