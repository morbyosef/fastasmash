use std::fs;
use std::path::PathBuf;
use crate::model::record::FastaRecord;
use crate::model::file::FastaFile;

pub fn parse_file(path: PathBuf, id: Option<&str>) -> FastaFile {
    let content = fs::read_to_string(&path).expect("Failed to open file");
    let records = content
        .split('>')
        .map(String::from)
        .filter(|s| !s.trim().is_empty())
        .map(FastaRecord::from)
        .filter(|r| id.map_or(true, |id| r.id.contains(id)))
        .collect();
    FastaFile { path, records }
}
