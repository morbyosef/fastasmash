use std::fs;
use std::path::PathBuf;
use crate::model::record::FastaRecord;
use crate::model::file::FastaFile;

pub fn parse_file(path: PathBuf) -> FastaFile {
    let content = fs::read_to_string(&path).unwrap();
    let records = content
        .split('>')
        .map(String::from)
        .filter(|s| !s.trim().is_empty())
        .map(FastaRecord::from)
        .collect();
    FastaFile { path, records }
}