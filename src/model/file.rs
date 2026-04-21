use std::path;
use crate::model::record::FastaRecord;

pub struct FastaFile {
    pub path: path::PathBuf,
    pub records: Vec<FastaRecord>,
}
