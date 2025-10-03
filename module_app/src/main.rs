mod adapters;
mod ingest;
mod normalize;

use std::path::Path;

fn main() {

    
let input_dir = Path::new("./input");
let ouput_dir = Path::new("./output");

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
    let output_path = ouput_dir.join(rel);

    let writer = adapters::adapter_for(&output_path);
    // Write the file unless there is an error
    if let Err(err) = writer.write(rec, &ouput_dir){
        eprintln!("Failed to write {}: {err}", output_path.display());
    }
}

}
