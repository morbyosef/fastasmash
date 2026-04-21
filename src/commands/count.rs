use crate::model::file::FastaFile;


/// Returns the number of records in the given fasta file.
pub fn count(fasta_file: FastaFile) -> u64 {
    return fasta_file.records.len() as u64;
}
