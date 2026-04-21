use crate::model::file::FastaFile;


/// Returns the number of records in the given fasta file.
pub fn count(fasta_file: &FastaFile) -> usize {
    fasta_file.records.len()
}
