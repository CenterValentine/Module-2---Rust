use crate::adapters::{adapter_for, FileRecord};
use std::fs;
use std::path::{Path,PathBuf};


// Return's file records from a stack.
pub fn ingest_directory(root: &Path, recursive: bool) -> Vec<FileRecord> {
    // new vector of file records named "records"
    let mut records = Vec::new();
    // A new vector borrowed path from PathBuf
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];

    // Loops through stack until None.
    while let Some(path) = stack.pop() {
        // Skips weird files without available metadata (permissoins, broken references, file in use, resource limits, corrupted in other ways).
        let Ok(meta) = fs::metadata(&path) else { continue };
        // checks if its a directory
        if meta.is_dir() {
            if let Ok(rd) = fs::read_dir(&path) {
                // .flatten() to skip errors
                for file_entry in rd.flatten() {
                    let p = file_entry.path();
                    // add to stack
                    if recursive {stack.push(p)}
// other valid files...
                    else if p.is_file() {
// add valid files to stack.
                if let Some(rec) = read_record(&p){
                    records.push(rec);
                    }
                }   
            }
        }
        // 
        if !recursive {
            if let Ok(rd) = fs::read_dir(&path){
                // .flatten() to skip errors
                for file_entry in rd.flatten() {
                    let p = file_entry.path();
                    if p.is_file() {
                        if let Some(rec) = read_record(&p) {
                            records.push(rec);
                        }
                    }
                }
            }
            break;
    }
} else if meta.is_file() {
    if let Some(rec) = read_record(&path) {
        records.push(rec);
    }
}
}
    records
}

pub fn read_record(path:&Path) -> Option<FileRecord> {
    let adapter = adapter_for(path);
    match adapter.read(path){
        Ok(rec) => Some(rec),
        Err(err) => {
            eprintln!("Skipping {}: {err}", path.display());
            None
        }
    }
}
