use std::fs;
use std::path::PathBuf;
use crate::model::record::FastaRecord;
use crate::model::file::FastaFile;

pub fn parse_file(path: PathBuf) -> FastaFile {
    let content = fs::read_to_string(&path).expect("Failed to open file");
    let records = content
        .split('>')
        .filter(|s| !s.trim().is_empty())
        .map(|s| FastaRecord::from(s.to_string()))
        .collect();
    FastaFile { path, records }
}

pub fn filter_by_id(file: FastaFile, id: &str) -> FastaFile {
    let records = file.records.into_iter()
        .filter(|r| r.id.contains(id))
        .collect();
    FastaFile { path: file.path, records }
}
