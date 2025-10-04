mod adapters;
mod ingest;
mod normalize;
use crate::adapters::{FileAdapter, Content};

use std::path::Path;

/*
1.cargo clean
2.	cargo check
3.	cargo build
*/


fn main() {

let input_dir = Path::new("./input");
let output_dir = Path::new("./output");

let mut records = ingest::ingest_directory(input_dir, false);

// ownership
records = records
// turns vector into an iterator. Iterate each record. This also takes ownership.
    .into_iter()
    // applies functions to each file
    .map(normalize::normalize)
    // gathers iterator back into vec (inferred) 
    .collect();

for rec in &records {
    // strip path and prefix. unwrap_or falls back to full path
    let rel = rec.path.strip_prefix(input_dir).unwrap_or(&rec.path);
    // Pathname
    let mut output_path = output_dir.join(rel);
    
    // sets extension to text file.
    output_path.set_extension("txt");

    let writer: Box<dyn FileAdapter + Send + Sync> = match rec.content {
        Content::Text(_)  => Box::new(adapters::text::TextFileAdapter),
        Content::Bytes(_) => Box::new(adapters::binary::BinaryFileAdapter),
    };
    // Write the file unless there is an error
    if let Err(err) = writer.write(rec, &output_path){
        eprintln!("Failed to write {}: {err}", output_path.display());
    }
}

}
