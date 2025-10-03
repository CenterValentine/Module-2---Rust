
use crate::adapters::{Content, FileType, FileRecord};


// For normalizing file records for downstream LLM ingestion.

pub fn normalize(mut rec: FileRecord) -> FileRecord {

    match (&rec.kind, &mut rec.content) {
        // trim, unite lines, remove whitespace, lowercase
        (FileType::Text, Content::Text(t)) => {
            let mut s = t.replace("\r\n", "\n").replace('\r', "\n");
            s = clean_whitespace(&s);
            s = s.trim().to_lowercase();
            *t = s;
    }
    (FileType::Binary, Content::Bytes(_)) => {

    }
    _ => {}
}
rec
}

fn clean_whitespace(input: &str) -> String {
    // input.len() is the byte length
    let mut out = String::with_capacity(input.len());
    let mut last_was_space = false;
    for ch in input.chars() {
        if !last_was_space {
            out.push(' ');
            last_was_space = true;
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out
}